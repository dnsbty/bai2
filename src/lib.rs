use chrono::{NaiveDate, NaiveTime};
use log::{debug, info};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Bai2File {
    pub continuations: Vec<Continuation>,
    pub file_control: FileControl,
    pub file_header: FileHeader,
    pub groups: Vec<Group>,
    #[serde(skip_serializing)]
    last_record_type: RecordType,
    #[serde(skip_serializing)]
    #[allow(dead_code)]
    raw: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
enum RecordType {
    File,
    Group,
    Account,
    Transaction,
}

fn parse_string(string: &str) -> String {
    string.trim().replace("/", "")
}

fn parse_currency(string: &str) -> String {
    return match parse_string(string).as_str() {
        "" => "USD".to_string(),
        c => c.to_string(),
    };
}

fn parse_date(string: &str) -> Option<NaiveDate> {
    let date = parse_string(string);
    let maybe_date = NaiveDate::parse_from_str(&date, "%y%m%d");
    match maybe_date {
        Ok(d) => Some(d),
        Err(_) => None,
    }
}

fn parse_time(string: &str) -> Option<String> {
    match parse_string(string).as_str() {
        "" => None,
        "2400" => Some("end of day".to_string()),
        "9999" => Some("end of day".to_string()),
        time => match NaiveTime::parse_from_str(time, "%H%M") {
            Ok(t) => Some(t.to_string()),
            Err(_) => None,
        },
    }
}

fn parse_int<T: FromStr>(string: &str) -> Option<T> {
    let number = string
        .trim()
        .replace("/", "")
        .trim_start_matches('0')
        .parse::<T>();

    match number {
        Ok(n) => Some(n),
        Err(_) => None,
    }
}

impl Bai2File {
    pub fn new(content: String) -> Bai2File {
        let _content = content.clone();
        let mut file = Bai2File {
            continuations: Vec::new(),
            file_control: FileControl::new(),
            file_header: FileHeader::new(),
            groups: Vec::new(),
            last_record_type: RecordType::File,
            raw: _content,
        };

        for linestr in content.lines() {
            let line = linestr.to_string();
            if line.len() < 2 {
                debug!("skipping empty line");
                continue;
            }

            let record_type = &line[0..2];
            match record_type {
                "01" => {
                    debug!("file header found");
                    file.file_header.parse(line);
                    file.last_record_type = RecordType::File;
                }
                "02" => {
                    debug!("group header found");
                    let group = Group {
                        accounts: Vec::new(),
                        continuations: Vec::new(),
                        control: GroupControl::new(),
                        header: GroupHeader::parse(line),
                    };
                    file.groups.push(group);
                    file.last_record_type = RecordType::Group;
                }
                "03" => {
                    debug!("account header found");
                    let account = Account {
                        continuations: Vec::new(),
                        control: AccountControl::new(),
                        header: AccountHeader::parse(line),
                        transactions: Vec::new(),
                    };
                    file.last_group().accounts.push(account);
                    file.last_record_type = RecordType::Account;
                }
                "16" => {
                    debug!("transaction found");
                    file.last_group()
                        .last_account()
                        .transactions
                        .push(Transaction::parse(line));
                    file.last_record_type = RecordType::Transaction;
                }
                "49" => {
                    debug!("account control found");
                    file.last_group().last_account().control.parse(line);
                    file.last_record_type = RecordType::Account;
                }
                "88" => {
                    debug!("continuation found");
                    match file.last_record_type {
                        RecordType::File => {
                            file.continuations.push(Continuation::parse(line));
                        }
                        RecordType::Group => {
                            file.last_group()
                                .continuations
                                .push(Continuation::parse(line));
                        }
                        RecordType::Account => {
                            file.last_group()
                                .last_account()
                                .continuations
                                .push(Continuation::parse(line));
                        }
                        RecordType::Transaction => {
                            file.last_group()
                                .last_account()
                                .last_transaction()
                                .continuations
                                .push(Continuation::parse(line));
                        }
                    }
                }
                "98" => {
                    debug!("group control found");
                    file.last_group().control.parse(line);
                    file.last_record_type = RecordType::Group;
                }
                "99" => {
                    debug!("file control found");
                    file.file_control.parse(line);
                    file.last_record_type = RecordType::File;
                    break;
                }
                _ => debug!("unknown record found"),
            }
        }
        info!("Done parsing file");
        return file;
    }

    fn last_group(&mut self) -> &mut Group {
        return self.groups.last_mut().unwrap();
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileHeader {
    pub block_size: Option<u8>,
    pub creation_date: Option<NaiveDate>,
    pub creation_time: Option<String>,
    pub file_id: String,
    pub physical_record_length: Option<u16>,
    pub receiver: String,
    pub sender: String,
    pub version_number: Option<u8>,
}

impl FileHeader {
    pub fn new() -> FileHeader {
        FileHeader {
            block_size: None,
            creation_date: None,
            creation_time: None,
            file_id: "".to_string(),
            physical_record_length: None,
            receiver: "".to_string(),
            sender: "".to_string(),
            version_number: None,
        }
    }

    fn parse(&mut self, line: String) {
        let fields = line.split(",").collect::<Vec<&str>>();

        self.sender = parse_string(fields[1]);
        self.receiver = parse_string(fields[2]);
        self.creation_date = parse_date(fields[3]);
        self.creation_time = parse_time(fields[4]);
        self.file_id = parse_string(fields[5]);
        self.physical_record_length = parse_int(fields[6]);
        self.block_size = parse_int(fields[7]);
        self.version_number = parse_int(fields[8]);
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileControl {
    number_of_groups: Option<u16>,
    number_of_records: Option<u16>,
    total: Option<u64>,
}

impl FileControl {
    pub fn new() -> FileControl {
        FileControl {
            number_of_groups: None,
            number_of_records: None,
            total: None,
        }
    }

    fn parse(&mut self, line: String) {
        let fields = line.split(",").collect::<Vec<&str>>();
        self.total = parse_int(fields[1]);
        self.number_of_groups = parse_int(fields[2]);
        self.number_of_records = parse_int(fields[3]);
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Group {
    accounts: Vec<Account>,
    continuations: Vec<Continuation>,
    control: GroupControl,
    header: GroupHeader,
}

impl Group {
    fn last_account(&mut self) -> &mut Account {
        return self.accounts.last_mut().unwrap();
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GroupHeader {
    as_of_date: Option<NaiveDate>,
    as_of_date_modifier: Option<AsOfDateModifier>,
    as_of_time: Option<String>,
    currency_code: String,
    originator: String,
    status: GroupStatus,
    ultimate_receiver: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum GroupStatus {
    Update,
    Deletion,
    Correction,
    TestOnly,
    Unknown(String),
}

impl GroupStatus {
    fn parse(value: &str) -> GroupStatus {
        match parse_string(value).as_str() {
            "1" => GroupStatus::Update,
            "2" => GroupStatus::Deletion,
            "3" => GroupStatus::Correction,
            "4" => GroupStatus::TestOnly,
            code => GroupStatus::Unknown(code.to_string()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum AsOfDateModifier {
    InterimPreviousDayData,
    FinalPreviousDayData,
    InterimSameDayData,
    FinalSameDayData,
}

impl AsOfDateModifier {
    fn parse(value: &str) -> Option<AsOfDateModifier> {
        match parse_string(value).as_str() {
            "1" => Some(AsOfDateModifier::InterimPreviousDayData),
            "2" => Some(AsOfDateModifier::FinalPreviousDayData),
            "3" => Some(AsOfDateModifier::InterimSameDayData),
            "4" => Some(AsOfDateModifier::FinalSameDayData),
            _ => None,
        }
    }
}

impl GroupHeader {
    fn parse(line: String) -> GroupHeader {
        let fields = line.split(",").collect::<Vec<&str>>();

        GroupHeader {
            as_of_date: parse_date(fields[4]),
            as_of_date_modifier: AsOfDateModifier::parse(fields[7]),
            as_of_time: parse_time(fields[5]),
            currency_code: parse_currency(fields[6]),
            originator: parse_string(fields[2]),
            status: GroupStatus::parse(fields[3]),
            ultimate_receiver: parse_string(fields[1]),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GroupControl {
    number_of_accounts: Option<u16>,
    number_of_records: Option<u16>,
    total: Option<u64>,
}

impl GroupControl {
    fn new() -> GroupControl {
        GroupControl {
            number_of_accounts: None,
            number_of_records: None,
            total: None,
        }
    }

    fn parse(&mut self, line: String) {
        let fields = line.split(",").collect::<Vec<&str>>();
        self.total = parse_int(fields[1]);
        self.number_of_accounts = parse_int(fields[2]);
        self.number_of_records = parse_int(fields[3]);
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Account {
    continuations: Vec<Continuation>,
    control: AccountControl,
    header: AccountHeader,
    transactions: Vec<Transaction>,
}

impl Account {
    fn last_transaction(&mut self) -> &mut Transaction {
        return self.transactions.last_mut().unwrap();
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum FundsType {
    Unknown,
    ImmediateAvailability,
    OneDayAvailability,
    TwoOrMoreDaysAvailability,
    ValueDated,
    DistributedAvailability,
}

impl FundsType {
    fn parse(value: &str) -> Option<FundsType> {
        match parse_string(value).as_str() {
            "0" => Some(FundsType::ImmediateAvailability),
            "1" => Some(FundsType::OneDayAvailability),
            "2" => Some(FundsType::TwoOrMoreDaysAvailability),
            "V" => Some(FundsType::ValueDated),
            "S" => Some(FundsType::DistributedAvailability),
            "D" => Some(FundsType::DistributedAvailability),
            _ => None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccountHeader {
    amount: Option<u64>,
    currency_code: String,
    customer_account_number: String,
    funds_type: Option<FundsType>,
    item_code: String,
    item_count: Option<u16>,
    account_type: Option<AccountType>,
}

impl AccountHeader {
    fn parse(line: String) -> AccountHeader {
        let fields = line.split(",").collect::<Vec<&str>>();

        let mut header = AccountHeader {
            amount: parse_int(fields[4]),
            currency_code: parse_currency(fields[2]),
            customer_account_number: parse_string(fields[1]),
            funds_type: None,
            item_code: parse_string(fields[5]),
            item_count: None,
            account_type: AccountType::parse(fields[3]),
        };

        if fields.len() > 6 {
            header.funds_type = FundsType::parse(fields[6]);
        }

        return header;
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum AccountType {
    AchNetPosition,
    AdjustedBalance,
    AdjustedBalanceMtd,
    AdjustedBalanceYtd,
    AdjustmentToBalances,
    AggregateBalanceAdjustments,
    Average1DayFloatMtd,
    Average1DayFloatYtd,
    Average2DayFloatMtd,
    Average2DayFloatYtd,
    AverageAdjustmentToBalancesMtd,
    AverageAdjustmentToBalancesYtd,
    AverageAvailablePreviousMonth,
    AverageClosingAvailableLastMonth,
    AverageClosingAvailableMtd,
    AverageClosingAvailableYtd,
    AverageClosingAvailableYtdLastMonth,
    AverageClosingLedgerMtd,
    AverageClosingLedgerPreviousMonth,
    AverageClosingLedgerYtd,
    AverageClosingLedgerYtdPreviousMonth,
    AverageCurrentAvailableMtd,
    AverageCurrentAvailableYtd,
    AverageOpeningAvailableMtd,
    AverageOpeningAvailableYtd,
    AverageOpeningLedgerMtd,
    AverageOpeningLedgerYtd,
    ClosingAvailable,
    ClosingLedger,
    CurrentAvailable,
    CurrentAvailableCrsSupressed,
    CurrentLedger,
    DisbursingOpeningAvailableBalance,
    FiveDayFloat,
    FloatAdjustment,
    FourDayFloat,
    LoanBalance,
    OneDayFloat,
    OpeningAvailable,
    OpeningAvailableAndTotalSameDayAchDtcDeposit,
    OpeningLedger,
    SixDayFloat,
    TargetBalance,
    ThreeOrMoreDaysFloat,
    TotalCredits,
    TotalDebits,
    TotalFloat,
    TotalInvestmentPosition,
    TransferCalculation,
    TwoOrMoreDaysFloat,
    ZeroDayFloat,
}

impl AccountType {
    fn parse(number: &str) -> Option<AccountType> {
        match number.trim().replace("/", "").as_str() {
            "010" => Some(AccountType::OpeningLedger),
            "011" => Some(AccountType::AverageOpeningLedgerMtd),
            "012" => Some(AccountType::AverageOpeningLedgerYtd),
            "015" => Some(AccountType::ClosingLedger),
            "020" => Some(AccountType::AverageClosingLedgerMtd),
            "021" => Some(AccountType::AverageClosingLedgerPreviousMonth),
            "022" => Some(AccountType::AggregateBalanceAdjustments),
            "024" => Some(AccountType::AverageClosingLedgerYtdPreviousMonth),
            "025" => Some(AccountType::AverageClosingLedgerYtd),
            "030" => Some(AccountType::CurrentLedger),
            "037" => Some(AccountType::AchNetPosition),
            "039" => Some(AccountType::OpeningAvailableAndTotalSameDayAchDtcDeposit),
            "040" => Some(AccountType::OpeningAvailable),
            "041" => Some(AccountType::AverageOpeningAvailableMtd),
            "042" => Some(AccountType::AverageOpeningAvailableYtd),
            "043" => Some(AccountType::AverageAvailablePreviousMonth),
            "044" => Some(AccountType::DisbursingOpeningAvailableBalance),
            "045" => Some(AccountType::ClosingAvailable),
            "050" => Some(AccountType::AverageClosingAvailableMtd),
            "051" => Some(AccountType::AverageClosingAvailableLastMonth),
            "054" => Some(AccountType::AverageClosingAvailableYtdLastMonth),
            "055" => Some(AccountType::AverageClosingAvailableYtd),
            "056" => Some(AccountType::LoanBalance),
            "057" => Some(AccountType::TotalInvestmentPosition),
            "059" => Some(AccountType::CurrentAvailableCrsSupressed),
            "060" => Some(AccountType::CurrentAvailable),
            "061" => Some(AccountType::AverageCurrentAvailableMtd),
            "062" => Some(AccountType::AverageCurrentAvailableYtd),
            "063" => Some(AccountType::TotalFloat),
            "065" => Some(AccountType::TargetBalance),
            "066" => Some(AccountType::AdjustedBalance),
            "067" => Some(AccountType::AdjustedBalanceMtd),
            "068" => Some(AccountType::AdjustedBalanceYtd),
            "070" => Some(AccountType::ZeroDayFloat),
            "072" => Some(AccountType::OneDayFloat),
            "073" => Some(AccountType::FloatAdjustment),
            "074" => Some(AccountType::TwoOrMoreDaysFloat),
            "075" => Some(AccountType::ThreeOrMoreDaysFloat),
            "076" => Some(AccountType::AdjustmentToBalances),
            "077" => Some(AccountType::AverageAdjustmentToBalancesMtd),
            "078" => Some(AccountType::AverageAdjustmentToBalancesYtd),
            "079" => Some(AccountType::FourDayFloat),
            "080" => Some(AccountType::FiveDayFloat),
            "081" => Some(AccountType::SixDayFloat),
            "082" => Some(AccountType::Average1DayFloatMtd),
            "083" => Some(AccountType::Average1DayFloatYtd),
            "084" => Some(AccountType::Average2DayFloatMtd),
            "085" => Some(AccountType::Average2DayFloatYtd),
            "086" => Some(AccountType::TransferCalculation),
            "100" => Some(AccountType::TotalCredits),
            "400" => Some(AccountType::TotalDebits),
            _ => None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccountControl {
    total: Option<u64>,
    number_of_records: Option<u32>,
}

impl AccountControl {
    fn new() -> AccountControl {
        AccountControl {
            number_of_records: None,
            total: None,
        }
    }

    fn parse(&mut self, line: String) {
        let fields = line.split(",").collect::<Vec<&str>>();
        self.total = parse_int(fields[1]);
        self.number_of_records = parse_int(fields[2]);
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transaction {
    amount: Option<u64>,
    bank_reference_number: String,
    continuations: Vec<Continuation>,
    customer_reference_number: String,
    direction: Direction,
    funds_type: Option<FundsType>,
    text: String,
    transaction_type: TransactionType,
}

impl Transaction {
    fn parse(line: String) -> Transaction {
        let fields = line.splitn(6, ",").collect::<Vec<&str>>();
        debug!("Parsing transaction: {:?}", fields);

        let (direction, transaction_type) = TransactionType::parse(fields[1]);

        let mut transaction = Transaction {
            amount: parse_int(fields[2]),
            bank_reference_number: parse_string(fields[4]),
            continuations: Vec::new(),
            customer_reference_number: parse_string(fields[5]),
            direction,
            funds_type: FundsType::parse(fields[3]),
            text: "".to_string(),
            transaction_type,
        };

        if fields.len() > 6 {
            transaction.text = parse_string(fields[6])
        }

        return transaction;
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
enum Direction {
    Credit,
    Debit,
    Unknown,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
enum TransactionType {
    AccountAnalysisFee,
    AccountHolderInitiatedAchDebit,
    AchConcentrationCredit,
    AchConcentrationDebit,
    AchCreditReceived,
    AchDebitReceived,
    AchDisbursementFundingDebit,
    AchReturnItemOrAdjustmentSettlement,
    AchReversalCredit,
    AchReversalDebit,
    AchSettlement,
    AchSettlementCredits,
    AchSettlementDebits,
    AdjustedTotalDisbursement,
    AmountAppliedToBuydown,
    AmountAppliedToDeferredInterestDetail,
    AmountAppliedToEscrow,
    AmountAppliedToInterest,
    AmountAppliedToLateCharges,
    AmountAppliedToMiscFees,
    AmountAppliedToPrincipal,
    AmountAppliedToServiceCharge,
    ArpDebit,
    AtmCredit,
    AtmDebit,
    BackValueAdjustment,
    BankersAcceptances,
    BankOriginatedDebit,
    BankPreparedDeposit,
    BondOperationsCredit,
    BondOperationsDebit,
    BookTransferCredit,
    BookTransferDebit,
    BrokerDebit,
    BrokerDeposit,
    CapitalChange,
    CashCenterCredit,
    CashCenterDebit,
    CashLetterAdjustment,
    CashLetterCredit,
    CashLetterDebit,
    CertifiedCheckDebit,
    CheckDepositPackage,
    CheckPaid,
    CheckPostedAndReturned,
    CheckReversal,
    ClearingSettlementCredit,
    ClearingSettlementDebit,
    CollectionOfDividends,
    CollectionOfInterestIncome,
    CommercialDeposit,
    CommercialPaper,
    Commission,
    Compensation,
    CorporateTradePaymentCredit,
    CorporateTradePaymentCredits,
    CorporateTradePaymentDebit,
    CorporateTradePaymentDebits,
    CorporateTradePaymentSettlement,
    CorrespondentBankDeposit,
    CorrespondentCollection,
    CorrespondentCollectionAdjustment,
    CorrespondentCollectionDebit,
    CouponCollectionDebit,
    CouponCollectionsBanks,
    Credit,
    CreditAdjustment,
    CreditReversal,
    CreditsNotDetailed,
    CumulativeChecksPaid,
    CumulativeCredits,
    CumulativeDebits,
    CumulativeZbaDebits,
    CumulativeZbaOrDisbursementCredits,
    CurrencyAndCoinDeposited,
    CurrencyAndCoinShipped,
    CurrentDayTotalLockboxDeposits,
    CustomerPayroll,
    CustomerTerminalInitiatedMoneyTransfer,
    DebitAdjustment,
    DebitAnyType,
    DebitReversal,
    DebitsNotDetailed,
    DepositCorrection,
    DepositCorrectionDebit,
    DepositedItemReturned,
    DepositReversal,
    DepositsSubjectToFloat,
    DisbursingFundingRequirement,
    DomesticCollection,
    Draft,
    DraftDeposit,
    DtcConcentrationCredit,
    DtcDebit,
    EdibanxCreditReceived,
    EdibanxCreditReturn,
    EdibanxReturnItemDebit,
    EdibanxSettlementDebit,
    EdiTransactionCredit,
    EdiTransactionDebit,
    EdiTransactionDebits,
    EstimatedTotalDisbursement,
    FederalReserveBankCommercialBankDebit,
    FederalReserveBankLetterDebit,
    FedFundsPurchased,
    FedFundsSold,
    FloatAdjustment,
    FoodStampAdjustment,
    FoodStampLetter,
    ForeignCheckPurchase,
    ForeignChecksDeposited,
    ForeignChecksPaid,
    ForeignCollectionCredit,
    ForeignCollectionDebit,
    ForeignExchangeDebit,
    ForeignExchangeOfCredit,
    ForeignLetterOfCredit,
    ForeignRemittanceCredit,
    ForeignRemittanceDebit,
    FrbCashLetterAutoChargeAdjustment,
    FrbCashLetterAutoChargeCredit,
    FrbCashLetterAutoChargeDebit,
    FrbFineSortAdjustment,
    FrbFineSortCashLetterCredit,
    FrbFineSortCashLetterDebit,
    FrbFreightPaymentDebits,
    FrbGovernmentCheckAdjustment,
    FrbGovernmentChecksCashLetterCredit,
    FrbGovernmentChecksCashLetterDebit,
    FrbPostalMoneyOrderAdjustment,
    FrbPostalMoneyOrderCredit,
    FrbPostalMoneyOrderDebit,
    FrbPresentmentEstimate,
    FrbStatementRecap,
    FreightPaymentCredit,
    FreightPaymentDebit,
    FuturesCredit,
    FuturesDebit,
    GrandTotalCreditsLessGrandTotalDebits,
    IncomingMoneyTransfer,
    IndividualAchReturnItem,
    IndividualAutomaticTransferCredit,
    IndividualAutomaticTransferDebit,
    IndividualBackValueCredit,
    IndividualBackValueDebit,
    IndividualBankCardDeposit,
    IndividualCollectionCredit,
    IndividualControlledDisbursingCredit,
    IndividualControlledDisbursingDebit,
    IndividualDtcDisbursingCredit,
    IndividualEscrowCredit,
    IndividualEscrowDebit,
    IndividualIncomingInternalMoneyTransfer,
    IndividualInternationalMoneyTransferCredit,
    IndividualInternationalMoneyTransferDebits,
    IndividualInvestmentPurchased,
    IndividualInvestmentSold,
    IndividualLoanDeposit,
    IndividualLoanPayment,
    IndividualOutgoingInternalMoneyTransfer,
    IndividualRejectedCredit,
    IndividualRejectedDebit,
    InterceptDebits,
    InterestAdjustmentCredit,
    InterestAdjustmentDebit,
    InterestCredit,
    InterestDebit,
    InterestMaturedPrincipalPayment,
    InternationalMoneyMarketTrading,
    InvestmentInterest,
    InvestmentSold,
    InvestmentsPurchased,
    ItemInAchDeposit,
    ItemInAchDisbursementOrDebit,
    ItemInBrokersDeposit,
    ItemInDtcDeposit,
    ItemInLockboxDeposit,
    ItemInPacDeposit,
    ItemizedCreditOver10000,
    ItemizedDebitOver10000,
    LateDebitsAfterNotification,
    LateDeposit,
    LetterOfCredit,
    LetterOfCreditDebit,
    ListPostCredits,
    ListPostDebit,
    ListPostDebits,
    LoanDisbursement,
    LoanParticipation,
    LockboxAdjustmentCredit,
    LockboxDebit,
    LockboxDeposit,
    MaturedFedFundsPurchased,
    MaturedRepurchaseOrder,
    MaturedReverseRepurchaseOrder,
    MaturityOfDebtSecurity,
    MiscellaneousAchCredit,
    MiscellaneousAchDebit,
    MiscellaneousCredit,
    MiscellaneousDebit,
    MiscellaneousFeeRefund,
    MiscellaneousFees,
    MiscellaneousInternationalCredit,
    MiscellaneousInternationalDebit,
    MiscellaneousSecurityCredit,
    MiscellaneousSecurityDebit,
    MoneyTransferAdjustment,
    MonthlyDividends,
    NetZeroBalanceAmount,
    OtherDeposit,
    OutgoingMoneyTransfer,
    Overdraft,
    OverdraftFee,
    PayableThroughDraft,
    PostingErrorCorrectionCredit,
    PostingErrorCorrectionDebit,
    PreauthorizedAchCredit,
    PreauthorizedAchDebit,
    PreauthorizedDraftCredit,
    PrincipalPaymentsCredit,
    PrincipalPaymentsDebit,
    PurchaseOfDebtSecurities,
    PurchaseOfEquitySecurities,
    RegularCollectionDebit,
    RePresentedCheckDeposit,
    ReturnItem,
    ReturnItemAdjustment,
    ReturnItemFee,
    SaleOfDebtSecurity,
    SaleOfEquitySecurity,
    SavingsBondLetterOrAdjustment,
    SavingsBondsSalesAdjustment,
    SecuritiesPurchased,
    SecuritiesSold,
    SecurityCollectionDebit,
    StandingOrder,
    SweepInterestIncome,
    SweepPrincipalBuy,
    SweepPrincipalSell,
    TodaysTotalDebits,
    TotalAchCredits,
    TotalAchDebits,
    TotalAchDisbursementFundingDebits,
    TotalAchDisbursingFundingCredits,
    TotalAchReturnItems,
    TotalAdjustmentCreditsYtd,
    TotalAmountOfSecuritiesPurchased,
    TotalAprDebits,
    TotalAtmCredits,
    TotalAtmDebits,
    TotalAutomaticTransferCredits,
    TotalAutomaticTransferDebits,
    TotalBackValueCredits,
    TotalBackValueDebits,
    TotalBankCardDeposits,
    TotalBankersAcceptanceCredits,
    TotalBankersAcceptancesDebit,
    TotalBankOriginatedDebits,
    TotalBankPreparedDeposits,
    TotalBookTransferCredits,
    TotalBookTransferDebits,
    TotalBrokerDebits,
    TotalBrokerDebitsChf,
    TotalBrokerDebitsFf,
    TotalBrokerDeposits,
    TotalBrokerDepositsChf,
    TotalBrokerDepositsFf,
    TotalCashCenterCredits,
    TotalCashCenterDebits,
    TotalCashLetterAdjustments,
    TotalCashLetterCredits,
    TotalCashLetterDebits,
    TotalCheckPaid,
    TotalCheckPaidCumulativeMtd,
    TotalChecksPostedAndReturned,
    TotalCollectionCredits,
    TotalCollectionDebit,
    TotalCommercialDeposits,
    TotalConcentrationCredits,
    TotalControlledDisbursingCredits,
    TotalControlledDisbursingDebits,
    TotalCreditAdjustment,
    TotalCreditAmountMtd,
    TotalCreditReversals,
    TotalCredits,
    TotalCreditsLessWireTransferAndReturnedChecks,
    TotalDebitAdjustments,
    TotalDebitAmountMtd,
    TotalDebitLessWireTransfersAndChargeBacks,
    TotalDebitReversals,
    TotalDebitsExcludingReturnedItems,
    TotalDepositedItemsReturned,
    TotalDisbursingChecksPaidEarlyAmount,
    TotalDisbursingChecksPaidLastAmount,
    TotalDisbursingChecksPaidLaterAmount,
    TotalDtcCredits,
    TotalDtcDebits,
    TotalDtcDisbursingCredits,
    TotalEscrowCredits,
    TotalEscrowDebits,
    TotalFederalReserveBankCommercialBankDebit,
    TotalFedFundsPurchased,
    TotalFedFundsSold,
    TotalForeignCheckPurchased,
    TotalFreightPaymentCredits,
    TotalFundsRequired,
    TotalIncomingMoneyTransfers,
    TotalInternationalCredits,
    TotalInternationalCreditsChf,
    TotalInternationalCreditsFf,
    TotalInternationalDebitChf,
    TotalInternationalDebitFf,
    TotalInternationalDebits,
    TotalInternationalMoneyTransferCredits,
    TotalInternationalMoneyTransferDebits,
    TotalInvestmentInterestDebits,
    TotalLettersOfCredit,
    TotalLoanPayment,
    TotalLoanPayments,
    TotalLoanProceeds,
    TotalLockboxDebits,
    TotalLockboxDeposits,
    TotalMiscellaneousCredits,
    TotalMiscellaneousDebits,
    TotalMiscellaneousDeposits,
    TotalMiscellaneousSecuritiesCreditsChf,
    TotalMiscellaneousSecuritiesCreditsFf,
    TotalMiscellaneousSecuritiesDbFf,
    TotalMiscellaneousSecuritiesDebitChf,
    TotalOtherCheckDeposits,
    TotalOutgoingMoneyTransfers,
    TotalPayableThroughDrafts,
    TotalPreauthorizedPaymentCredits,
    TotalRejectedCredits,
    TotalRejectedDebits,
    TotalSecuritiesInterest,
    TotalSecuritiesInterestChf,
    TotalSecuritiesInterestFf,
    TotalSecuritiesMatured,
    TotalSecuritiesMaturedChf,
    TotalSecuritiesMaturedFf,
    TotalSecuritiesPurchasedChf,
    TotalSecuritiesPurchasedFf,
    TotalSecuritiesSold,
    TotalSecuritiesSoldChf,
    TotalSecuritiesSoldFf,
    TotalSecurityCredits,
    TotalSecurityDebits,
    TotalTrustCredits,
    TotalTrustDebits,
    TotalUniversalCredits,
    TotalUniversalDebits,
    TotalValueDatedFunds,
    TotalWireTransfersInCHF,
    TotalWireTransfersInFF,
    TotalWireTransfersOutChf,
    TotalWireTransfersOutFf,
    TotalYtdAdjustment,
    TotalZbaCredits,
    TotalZbaDebits,
    TransferCalculationDebit,
    TransferOfTreasuryCredit,
    TransferOfTreasuryDebit,
    TreasuryTaxAndLoanCredit,
    TreasuryTaxAndLoanDebit,
    TrustCredit,
    TrustDebit,
    UniversalCredit,
    UniversalDebit,
    Unknown,
    YtdAdjustmentCredit,
    YtdAdjustmentDebit,
    ZbaCredit,
    ZbaCreditAdjustment,
    ZbaCreditTransfer,
    ZbaDebit,
    ZbaDebitAdjustment,
    ZbaDebitTransfer,
    ZbaFloatAdjustment,
}

impl TransactionType {
    fn parse(type_code: &str) -> (Direction, TransactionType) {
        match type_code.trim().replace("/", "").as_str() {
            "100" => (Direction::Credit, TransactionType::TotalCredits),
            "101" => (Direction::Credit, TransactionType::TotalCreditAmountMtd),
            "105" => (Direction::Credit, TransactionType::CreditsNotDetailed),
            "106" => (Direction::Credit, TransactionType::DepositsSubjectToFloat),
            "107" => (
                Direction::Credit,
                TransactionType::TotalAdjustmentCreditsYtd,
            ),
            "108" => (Direction::Credit, TransactionType::Credit),
            "109" => (
                Direction::Credit,
                TransactionType::CurrentDayTotalLockboxDeposits,
            ),
            "110" => (Direction::Credit, TransactionType::TotalLockboxDeposits),
            "115" => (Direction::Credit, TransactionType::LockboxDeposit),
            "116" => (Direction::Credit, TransactionType::ItemInLockboxDeposit),
            "118" => (Direction::Credit, TransactionType::LockboxAdjustmentCredit),
            "120" => (Direction::Credit, TransactionType::EdiTransactionCredit),
            "121" => (Direction::Credit, TransactionType::EdiTransactionCredit),
            "122" => (Direction::Credit, TransactionType::EdibanxCreditReceived),
            "123" => (Direction::Credit, TransactionType::EdibanxCreditReturn),
            "130" => (
                Direction::Credit,
                TransactionType::TotalConcentrationCredits,
            ),
            "131" => (Direction::Credit, TransactionType::TotalDtcCredits),
            "135" => (Direction::Credit, TransactionType::DtcConcentrationCredit),
            "136" => (Direction::Credit, TransactionType::ItemInDtcDeposit),
            "140" => (Direction::Credit, TransactionType::TotalAchCredits),
            "142" => (Direction::Credit, TransactionType::AchCreditReceived),
            "143" => (Direction::Credit, TransactionType::ItemInAchDeposit),
            "145" => (Direction::Credit, TransactionType::AchConcentrationCredit),
            "146" => (Direction::Credit, TransactionType::TotalBankCardDeposits),
            "147" => (
                Direction::Credit,
                TransactionType::IndividualBankCardDeposit,
            ),
            "150" => (
                Direction::Credit,
                TransactionType::TotalPreauthorizedPaymentCredits,
            ),
            "155" => (Direction::Credit, TransactionType::PreauthorizedDraftCredit),
            "156" => (Direction::Credit, TransactionType::ItemInPacDeposit),
            "160" => (
                Direction::Credit,
                TransactionType::TotalAchDisbursingFundingCredits,
            ),
            "162" => (
                Direction::Credit,
                TransactionType::CorporateTradePaymentSettlement,
            ),
            "163" => (
                Direction::Credit,
                TransactionType::CorporateTradePaymentCredits,
            ),
            "164" => (
                Direction::Credit,
                TransactionType::CorporateTradePaymentCredit,
            ),
            "165" => (Direction::Credit, TransactionType::PreauthorizedAchCredit),
            "166" => (Direction::Credit, TransactionType::AchSettlement),
            "167" => (Direction::Credit, TransactionType::AchSettlementCredits),
            "168" => (
                Direction::Credit,
                TransactionType::AchReturnItemOrAdjustmentSettlement,
            ),
            "169" => (Direction::Credit, TransactionType::MiscellaneousAchCredit),
            "170" => (Direction::Credit, TransactionType::TotalOtherCheckDeposits),
            "171" => (Direction::Credit, TransactionType::IndividualLoanDeposit),
            "172" => (Direction::Credit, TransactionType::DepositCorrection),
            "173" => (Direction::Credit, TransactionType::BankPreparedDeposit),
            "174" => (Direction::Credit, TransactionType::OtherDeposit),
            "175" => (Direction::Credit, TransactionType::CheckDepositPackage),
            "176" => (Direction::Credit, TransactionType::RePresentedCheckDeposit),
            "178" => (Direction::Credit, TransactionType::ListPostCredits),
            "180" => (Direction::Credit, TransactionType::TotalLoanProceeds),
            "182" => (
                Direction::Credit,
                TransactionType::TotalBankPreparedDeposits,
            ),
            "184" => (Direction::Credit, TransactionType::DraftDeposit),
            "185" => (
                Direction::Credit,
                TransactionType::TotalMiscellaneousDeposits,
            ),
            "186" => (Direction::Credit, TransactionType::TotalCashLetterCredits),
            "187" => (Direction::Credit, TransactionType::CashLetterCredit),
            "188" => (
                Direction::Credit,
                TransactionType::TotalCashLetterAdjustments,
            ),
            "189" => (Direction::Credit, TransactionType::CashLetterAdjustment),
            "190" => (
                Direction::Credit,
                TransactionType::TotalIncomingMoneyTransfers,
            ),
            "191" => (
                Direction::Credit,
                TransactionType::IndividualIncomingInternalMoneyTransfer,
            ),
            "195" => (Direction::Credit, TransactionType::IncomingMoneyTransfer),
            "196" => (Direction::Credit, TransactionType::MoneyTransferAdjustment),
            "198" => (Direction::Credit, TransactionType::Compensation),
            "200" => (
                Direction::Credit,
                TransactionType::TotalAutomaticTransferCredits,
            ),
            "201" => (
                Direction::Credit,
                TransactionType::IndividualAutomaticTransferCredit,
            ),
            "202" => (Direction::Credit, TransactionType::BondOperationsCredit),
            "205" => (Direction::Credit, TransactionType::TotalBookTransferCredits),
            "206" => (Direction::Credit, TransactionType::BookTransferCredit),
            "207" => (
                Direction::Credit,
                TransactionType::TotalInternationalMoneyTransferCredits,
            ),
            "208" => (
                Direction::Credit,
                TransactionType::IndividualInternationalMoneyTransferCredit,
            ),
            "210" => (
                Direction::Credit,
                TransactionType::TotalInternationalCredits,
            ),
            "212" => (Direction::Credit, TransactionType::ForeignLetterOfCredit),
            "213" => (Direction::Credit, TransactionType::LetterOfCredit),
            "214" => (Direction::Credit, TransactionType::ForeignExchangeOfCredit),
            "215" => (Direction::Credit, TransactionType::TotalLettersOfCredit),
            "216" => (Direction::Credit, TransactionType::ForeignRemittanceCredit),
            "218" => (Direction::Credit, TransactionType::ForeignCollectionCredit),
            "221" => (Direction::Credit, TransactionType::ForeignCheckPurchase),
            "222" => (Direction::Credit, TransactionType::ForeignChecksDeposited),
            "224" => (Direction::Credit, TransactionType::Commission),
            "226" => (
                Direction::Credit,
                TransactionType::InternationalMoneyMarketTrading,
            ),
            "227" => (Direction::Credit, TransactionType::StandingOrder),
            "229" => (
                Direction::Credit,
                TransactionType::MiscellaneousInternationalCredit,
            ),
            "230" => (Direction::Credit, TransactionType::TotalSecurityCredits),
            "231" => (Direction::Credit, TransactionType::TotalCollectionCredits),
            "232" => (Direction::Credit, TransactionType::SaleOfDebtSecurity),
            "233" => (Direction::Credit, TransactionType::SecuritiesSold),
            "234" => (Direction::Credit, TransactionType::SaleOfEquitySecurity),
            "235" => (
                Direction::Credit,
                TransactionType::MaturedReverseRepurchaseOrder,
            ),
            "236" => (Direction::Credit, TransactionType::MaturityOfDebtSecurity),
            "237" => (
                Direction::Credit,
                TransactionType::IndividualCollectionCredit,
            ),
            "238" => (Direction::Credit, TransactionType::CollectionOfDividends),
            "239" => (
                Direction::Credit,
                TransactionType::TotalBankersAcceptanceCredits,
            ),
            "240" => (Direction::Credit, TransactionType::CouponCollectionsBanks),
            "241" => (Direction::Credit, TransactionType::BankersAcceptances),
            "242" => (
                Direction::Credit,
                TransactionType::CollectionOfInterestIncome,
            ),
            "243" => (Direction::Credit, TransactionType::MaturedFedFundsPurchased),
            "244" => (
                Direction::Credit,
                TransactionType::InterestMaturedPrincipalPayment,
            ),
            "245" => (Direction::Credit, TransactionType::MonthlyDividends),
            "246" => (Direction::Credit, TransactionType::CommercialPaper),
            "247" => (Direction::Credit, TransactionType::CapitalChange),
            "248" => (
                Direction::Credit,
                TransactionType::SavingsBondsSalesAdjustment,
            ),
            "249" => (
                Direction::Credit,
                TransactionType::MiscellaneousSecurityCredit,
            ),
            "250" => (
                Direction::Credit,
                TransactionType::TotalChecksPostedAndReturned,
            ),
            "251" => (Direction::Credit, TransactionType::TotalDebitReversals),
            "252" => (Direction::Credit, TransactionType::DebitReversal),
            "254" => (
                Direction::Credit,
                TransactionType::PostingErrorCorrectionCredit,
            ),
            "255" => (Direction::Credit, TransactionType::CheckPostedAndReturned),
            "256" => (Direction::Credit, TransactionType::TotalAchReturnItems),
            "257" => (Direction::Credit, TransactionType::IndividualAchReturnItem),
            "258" => (Direction::Credit, TransactionType::AchReversalCredit),
            "260" => (Direction::Credit, TransactionType::TotalRejectedCredits),
            "261" => (Direction::Credit, TransactionType::IndividualRejectedCredit),
            "263" => (Direction::Credit, TransactionType::Overdraft),
            "266" => (Direction::Credit, TransactionType::ReturnItem),
            "268" => (Direction::Credit, TransactionType::ReturnItemAdjustment),
            "270" => (Direction::Credit, TransactionType::TotalZbaCredits),
            "271" => (Direction::Credit, TransactionType::NetZeroBalanceAmount),
            "274" => (
                Direction::Credit,
                TransactionType::CumulativeZbaOrDisbursementCredits,
            ),
            "275" => (Direction::Credit, TransactionType::ZbaCredit),
            "276" => (Direction::Credit, TransactionType::ZbaFloatAdjustment),
            "277" => (Direction::Credit, TransactionType::ZbaCreditTransfer),
            "278" => (Direction::Credit, TransactionType::ZbaCreditAdjustment),
            "280" => (
                Direction::Credit,
                TransactionType::TotalControlledDisbursingCredits,
            ),
            "281" => (
                Direction::Credit,
                TransactionType::IndividualControlledDisbursingCredit,
            ),
            "285" => (
                Direction::Credit,
                TransactionType::TotalDtcDisbursingCredits,
            ),
            "286" => (
                Direction::Credit,
                TransactionType::IndividualDtcDisbursingCredit,
            ),
            "294" => (Direction::Credit, TransactionType::TotalAtmCredits),
            "295" => (Direction::Credit, TransactionType::AtmCredit),
            "301" => (Direction::Credit, TransactionType::CommercialDeposit),
            "302" => (Direction::Credit, TransactionType::CorrespondentBankDeposit),
            "303" => (Direction::Credit, TransactionType::TotalWireTransfersInFF),
            "304" => (Direction::Credit, TransactionType::TotalWireTransfersInCHF),
            "305" => (Direction::Credit, TransactionType::TotalFedFundsSold),
            "306" => (Direction::Credit, TransactionType::FedFundsSold),
            "307" => (Direction::Credit, TransactionType::TotalTrustCredits),
            "308" => (Direction::Credit, TransactionType::TrustCredit),
            "309" => (Direction::Credit, TransactionType::TotalValueDatedFunds),
            "310" => (Direction::Credit, TransactionType::TotalCommercialDeposits),
            "315" => (
                Direction::Credit,
                TransactionType::TotalInternationalCreditsFf,
            ),
            "316" => (
                Direction::Credit,
                TransactionType::TotalInternationalCreditsChf,
            ),
            "318" => (
                Direction::Credit,
                TransactionType::TotalForeignCheckPurchased,
            ),
            "319" => (Direction::Credit, TransactionType::LateDeposit),
            "320" => (Direction::Credit, TransactionType::TotalSecuritiesSoldFf),
            "321" => (Direction::Credit, TransactionType::TotalSecuritiesSoldChf),
            "324" => (Direction::Credit, TransactionType::TotalSecuritiesMaturedFf),
            "325" => (
                Direction::Credit,
                TransactionType::TotalSecuritiesMaturedChf,
            ),
            "326" => (Direction::Credit, TransactionType::TotalSecuritiesInterest),
            "327" => (Direction::Credit, TransactionType::TotalSecuritiesMatured),
            "328" => (
                Direction::Credit,
                TransactionType::TotalSecuritiesInterestFf,
            ),
            "329" => (
                Direction::Credit,
                TransactionType::TotalSecuritiesInterestChf,
            ),
            "330" => (Direction::Credit, TransactionType::TotalEscrowCredits),
            "331" => (Direction::Credit, TransactionType::IndividualEscrowCredit),
            "332" => (
                Direction::Credit,
                TransactionType::TotalMiscellaneousSecuritiesCreditsFf,
            ),
            "336" => (
                Direction::Credit,
                TransactionType::TotalMiscellaneousSecuritiesCreditsChf,
            ),
            "338" => (Direction::Credit, TransactionType::TotalSecuritiesSold),
            "340" => (Direction::Credit, TransactionType::TotalBrokerDeposits),
            "341" => (Direction::Credit, TransactionType::TotalBrokerDepositsFf),
            "342" => (Direction::Credit, TransactionType::BrokerDeposit),
            "343" => (Direction::Credit, TransactionType::TotalBrokerDepositsChf),
            "344" => (
                Direction::Credit,
                TransactionType::IndividualBackValueCredit,
            ),
            "345" => (Direction::Credit, TransactionType::ItemInBrokersDeposit),
            "346" => (Direction::Credit, TransactionType::SweepInterestIncome),
            "347" => (Direction::Credit, TransactionType::SweepPrincipalSell),
            "348" => (Direction::Credit, TransactionType::FuturesCredit),
            "349" => (Direction::Credit, TransactionType::PrincipalPaymentsCredit),
            "350" => (Direction::Credit, TransactionType::InvestmentSold),
            "351" => (Direction::Credit, TransactionType::IndividualInvestmentSold),
            "352" => (Direction::Credit, TransactionType::TotalCashCenterCredits),
            "353" => (Direction::Credit, TransactionType::CashCenterCredit),
            "354" => (Direction::Credit, TransactionType::InterestCredit),
            "355" => (Direction::Credit, TransactionType::InvestmentInterest),
            "356" => (Direction::Credit, TransactionType::TotalCreditAdjustment),
            "357" => (Direction::Credit, TransactionType::CreditAdjustment),
            "358" => (Direction::Credit, TransactionType::YtdAdjustmentCredit),
            "359" => (Direction::Credit, TransactionType::InterestAdjustmentCredit),
            "360" => (
                Direction::Credit,
                TransactionType::TotalCreditsLessWireTransferAndReturnedChecks,
            ),
            "361" => (
                Direction::Credit,
                TransactionType::GrandTotalCreditsLessGrandTotalDebits,
            ),
            "362" => (Direction::Credit, TransactionType::CorrespondentCollection),
            "363" => (
                Direction::Credit,
                TransactionType::CorrespondentCollectionAdjustment,
            ),
            "364" => (Direction::Credit, TransactionType::LoanParticipation),
            "366" => (Direction::Credit, TransactionType::CurrencyAndCoinDeposited),
            "367" => (Direction::Credit, TransactionType::FoodStampLetter),
            "368" => (Direction::Credit, TransactionType::FoodStampAdjustment),
            "369" => (Direction::Credit, TransactionType::ClearingSettlementCredit),
            "370" => (Direction::Credit, TransactionType::TotalBackValueCredits),
            "372" => (Direction::Credit, TransactionType::BackValueAdjustment),
            "373" => (Direction::Credit, TransactionType::CustomerPayroll),
            "374" => (Direction::Credit, TransactionType::FrbStatementRecap),
            "376" => (
                Direction::Credit,
                TransactionType::SavingsBondLetterOrAdjustment,
            ),
            "377" => (Direction::Credit, TransactionType::TreasuryTaxAndLoanCredit),
            "378" => (Direction::Credit, TransactionType::TransferOfTreasuryCredit),
            "379" => (
                Direction::Credit,
                TransactionType::FrbGovernmentChecksCashLetterCredit,
            ),
            "381" => (
                Direction::Credit,
                TransactionType::FrbGovernmentCheckAdjustment,
            ),
            "382" => (
                Direction::Credit,
                TransactionType::FrbPostalMoneyOrderCredit,
            ),
            "383" => (
                Direction::Credit,
                TransactionType::FrbPostalMoneyOrderAdjustment,
            ),
            "384" => (
                Direction::Credit,
                TransactionType::FrbCashLetterAutoChargeCredit,
            ),
            "385" => (Direction::Credit, TransactionType::TotalUniversalCredits),
            "386" => (
                Direction::Credit,
                TransactionType::FrbCashLetterAutoChargeAdjustment,
            ),
            "387" => (
                Direction::Credit,
                TransactionType::FrbFineSortCashLetterCredit,
            ),
            "388" => (Direction::Credit, TransactionType::FrbFineSortAdjustment),
            "389" => (
                Direction::Credit,
                TransactionType::TotalFreightPaymentCredits,
            ),
            "390" => (
                Direction::Credit,
                TransactionType::TotalMiscellaneousCredits,
            ),
            "391" => (Direction::Credit, TransactionType::UniversalCredit),
            "392" => (Direction::Credit, TransactionType::FreightPaymentCredit),
            "393" => (Direction::Credit, TransactionType::ItemizedCreditOver10000),
            "394" => (Direction::Credit, TransactionType::CumulativeCredits),
            "395" => (Direction::Credit, TransactionType::CheckReversal),
            "397" => (Direction::Credit, TransactionType::FloatAdjustment),
            "398" => (Direction::Credit, TransactionType::MiscellaneousFeeRefund),
            "399" => (Direction::Credit, TransactionType::MiscellaneousCredit),
            "401" => (Direction::Debit, TransactionType::TotalDebitAmountMtd),
            "403" => (Direction::Debit, TransactionType::TodaysTotalDebits),
            "405" => (
                Direction::Debit,
                TransactionType::TotalDebitLessWireTransfersAndChargeBacks,
            ),
            "406" => (Direction::Debit, TransactionType::DebitsNotDetailed),
            "408" => (Direction::Debit, TransactionType::FloatAdjustment),
            "409" => (Direction::Debit, TransactionType::DebitAnyType),
            "410" => (Direction::Debit, TransactionType::TotalYtdAdjustment),
            "412" => (
                Direction::Debit,
                TransactionType::TotalDebitsExcludingReturnedItems,
            ),
            "415" => (Direction::Debit, TransactionType::LockboxDebit),
            "416" => (Direction::Debit, TransactionType::TotalLockboxDebits),
            "420" => (Direction::Debit, TransactionType::EdiTransactionDebits),
            "421" => (Direction::Debit, TransactionType::EdiTransactionDebit),
            "422" => (Direction::Debit, TransactionType::EdibanxSettlementDebit),
            "423" => (Direction::Debit, TransactionType::EdibanxReturnItemDebit),
            "430" => (Direction::Debit, TransactionType::TotalPayableThroughDrafts),
            "435" => (Direction::Debit, TransactionType::PayableThroughDraft),
            "445" => (Direction::Debit, TransactionType::AchConcentrationDebit),
            "446" => (
                Direction::Debit,
                TransactionType::TotalAchDisbursementFundingDebits,
            ),
            "447" => (
                Direction::Debit,
                TransactionType::AchDisbursementFundingDebit,
            ),
            "450" => (Direction::Debit, TransactionType::TotalAchDebits),
            "451" => (Direction::Debit, TransactionType::AchDebitReceived),
            "452" => (
                Direction::Debit,
                TransactionType::ItemInAchDisbursementOrDebit,
            ),
            "455" => (Direction::Debit, TransactionType::PreauthorizedAchDebit),
            "462" => (
                Direction::Debit,
                TransactionType::AccountHolderInitiatedAchDebit,
            ),
            "463" => (
                Direction::Debit,
                TransactionType::CorporateTradePaymentDebits,
            ),
            "464" => (
                Direction::Debit,
                TransactionType::CorporateTradePaymentDebit,
            ),
            "465" => (
                Direction::Debit,
                TransactionType::CorporateTradePaymentSettlement,
            ),
            "466" => (Direction::Debit, TransactionType::AchSettlement),
            "467" => (Direction::Debit, TransactionType::AchSettlementDebits),
            "468" => (
                Direction::Debit,
                TransactionType::AchReturnItemOrAdjustmentSettlement,
            ),
            "469" => (Direction::Debit, TransactionType::MiscellaneousAchDebit),
            "470" => (Direction::Debit, TransactionType::TotalCheckPaid),
            "471" => (
                Direction::Debit,
                TransactionType::TotalCheckPaidCumulativeMtd,
            ),
            "472" => (Direction::Debit, TransactionType::CumulativeChecksPaid),
            "474" => (Direction::Debit, TransactionType::CertifiedCheckDebit),
            "475" => (Direction::Debit, TransactionType::CheckPaid),
            "476" => (
                Direction::Debit,
                TransactionType::FederalReserveBankLetterDebit,
            ),
            "477" => (Direction::Debit, TransactionType::BankOriginatedDebit),
            "478" => (Direction::Debit, TransactionType::ListPostDebits),
            "479" => (Direction::Debit, TransactionType::ListPostDebit),
            "480" => (Direction::Debit, TransactionType::TotalLoanPayments),
            "481" => (Direction::Debit, TransactionType::IndividualLoanPayment),
            "482" => (Direction::Debit, TransactionType::TotalBankOriginatedDebits),
            "484" => (Direction::Debit, TransactionType::Draft),
            "485" => (Direction::Debit, TransactionType::DtcDebit),
            "486" => (Direction::Debit, TransactionType::TotalCashLetterDebits),
            "487" => (Direction::Debit, TransactionType::CashLetterDebit),
            "489" => (Direction::Debit, TransactionType::CashLetterAdjustment),
            "490" => (
                Direction::Debit,
                TransactionType::TotalOutgoingMoneyTransfers,
            ),
            "491" => (
                Direction::Debit,
                TransactionType::IndividualOutgoingInternalMoneyTransfer,
            ),
            "493" => (
                Direction::Debit,
                TransactionType::CustomerTerminalInitiatedMoneyTransfer,
            ),
            "495" => (Direction::Debit, TransactionType::OutgoingMoneyTransfer),
            "496" => (Direction::Debit, TransactionType::MoneyTransferAdjustment),
            "498" => (Direction::Debit, TransactionType::Compensation),
            "500" => (
                Direction::Debit,
                TransactionType::TotalAutomaticTransferDebits,
            ),
            "501" => (
                Direction::Debit,
                TransactionType::IndividualAutomaticTransferDebit,
            ),
            "502" => (Direction::Debit, TransactionType::BondOperationsDebit),
            "505" => (Direction::Debit, TransactionType::TotalBookTransferDebits),
            "506" => (Direction::Debit, TransactionType::BookTransferDebit),
            "507" => (
                Direction::Debit,
                TransactionType::TotalInternationalMoneyTransferDebits,
            ),
            "508" => (
                Direction::Debit,
                TransactionType::IndividualInternationalMoneyTransferDebits,
            ),
            "510" => (Direction::Debit, TransactionType::TotalInternationalDebits),
            "512" => (Direction::Debit, TransactionType::LetterOfCreditDebit),
            "513" => (Direction::Debit, TransactionType::LetterOfCredit),
            "514" => (Direction::Debit, TransactionType::ForeignExchangeDebit),
            "515" => (Direction::Debit, TransactionType::TotalLettersOfCredit),
            "516" => (Direction::Debit, TransactionType::ForeignRemittanceDebit),
            "518" => (Direction::Debit, TransactionType::ForeignCollectionDebit),
            "522" => (Direction::Debit, TransactionType::ForeignChecksPaid),
            "524" => (Direction::Debit, TransactionType::Commission),
            "526" => (
                Direction::Debit,
                TransactionType::InternationalMoneyMarketTrading,
            ),
            "527" => (Direction::Debit, TransactionType::StandingOrder),
            "529" => (
                Direction::Debit,
                TransactionType::MiscellaneousInternationalDebit,
            ),
            "530" => (Direction::Debit, TransactionType::TotalSecurityDebits),
            "531" => (Direction::Debit, TransactionType::SecuritiesPurchased),
            "532" => (
                Direction::Debit,
                TransactionType::TotalAmountOfSecuritiesPurchased,
            ),
            "533" => (Direction::Debit, TransactionType::SecurityCollectionDebit),
            "534" => (
                Direction::Debit,
                TransactionType::TotalMiscellaneousSecuritiesDbFf,
            ),
            "535" => (
                Direction::Debit,
                TransactionType::PurchaseOfEquitySecurities,
            ),
            "536" => (
                Direction::Debit,
                TransactionType::TotalMiscellaneousSecuritiesDebitChf,
            ),
            "537" => (Direction::Debit, TransactionType::TotalCollectionDebit),
            "538" => (Direction::Debit, TransactionType::MaturedRepurchaseOrder),
            "539" => (
                Direction::Debit,
                TransactionType::TotalBankersAcceptancesDebit,
            ),
            "540" => (Direction::Debit, TransactionType::CouponCollectionDebit),
            "541" => (Direction::Debit, TransactionType::BankersAcceptances),
            "542" => (Direction::Debit, TransactionType::PurchaseOfDebtSecurities),
            "543" => (Direction::Debit, TransactionType::DomesticCollection),
            "544" => (
                Direction::Debit,
                TransactionType::InterestMaturedPrincipalPayment,
            ),
            "546" => (Direction::Debit, TransactionType::CommercialPaper),
            "547" => (Direction::Debit, TransactionType::CapitalChange),
            "548" => (
                Direction::Debit,
                TransactionType::SavingsBondsSalesAdjustment,
            ),
            "549" => (
                Direction::Debit,
                TransactionType::MiscellaneousSecurityDebit,
            ),
            "550" => (
                Direction::Debit,
                TransactionType::TotalDepositedItemsReturned,
            ),
            "551" => (Direction::Debit, TransactionType::TotalCreditReversals),
            "552" => (Direction::Debit, TransactionType::CreditReversal),
            "554" => (
                Direction::Debit,
                TransactionType::PostingErrorCorrectionDebit,
            ),
            "555" => (Direction::Debit, TransactionType::DepositedItemReturned),
            "556" => (Direction::Debit, TransactionType::TotalAchReturnItems),
            "557" => (Direction::Debit, TransactionType::IndividualAchReturnItem),
            "558" => (Direction::Debit, TransactionType::AchReversalDebit),
            "560" => (Direction::Debit, TransactionType::TotalRejectedDebits),
            "561" => (Direction::Debit, TransactionType::IndividualRejectedDebit),
            "563" => (Direction::Debit, TransactionType::Overdraft),
            "564" => (Direction::Debit, TransactionType::OverdraftFee),
            "566" => (Direction::Debit, TransactionType::ReturnItem),
            "567" => (Direction::Debit, TransactionType::ReturnItemFee),
            "568" => (Direction::Debit, TransactionType::ReturnItemAdjustment),
            "570" => (Direction::Debit, TransactionType::TotalZbaDebits),
            "574" => (Direction::Debit, TransactionType::CumulativeZbaDebits),
            "575" => (Direction::Debit, TransactionType::ZbaDebit),
            "577" => (Direction::Debit, TransactionType::ZbaDebitTransfer),
            "578" => (Direction::Debit, TransactionType::ZbaDebitAdjustment),
            "580" => (
                Direction::Debit,
                TransactionType::TotalControlledDisbursingDebits,
            ),
            "581" => (
                Direction::Debit,
                TransactionType::IndividualControlledDisbursingDebit,
            ),
            "583" => (
                Direction::Debit,
                TransactionType::TotalDisbursingChecksPaidEarlyAmount,
            ),
            "584" => (
                Direction::Debit,
                TransactionType::TotalDisbursingChecksPaidLaterAmount,
            ),
            "585" => (
                Direction::Debit,
                TransactionType::DisbursingFundingRequirement,
            ),
            "586" => (Direction::Debit, TransactionType::FrbPresentmentEstimate),
            "587" => (
                Direction::Debit,
                TransactionType::LateDebitsAfterNotification,
            ),
            "588" => (
                Direction::Debit,
                TransactionType::TotalDisbursingChecksPaidLastAmount,
            ),
            "590" => (Direction::Debit, TransactionType::TotalDtcDebits),
            "594" => (Direction::Debit, TransactionType::TotalAtmDebits),
            "595" => (Direction::Debit, TransactionType::AtmDebit),
            "596" => (Direction::Debit, TransactionType::TotalAprDebits),
            "597" => (Direction::Debit, TransactionType::ArpDebit),
            "601" => (
                Direction::Debit,
                TransactionType::EstimatedTotalDisbursement,
            ),
            "602" => (Direction::Debit, TransactionType::AdjustedTotalDisbursement),
            "610" => (Direction::Debit, TransactionType::TotalFundsRequired),
            "611" => (Direction::Debit, TransactionType::TotalWireTransfersOutChf),
            "612" => (Direction::Debit, TransactionType::TotalWireTransfersOutFf),
            "613" => (
                Direction::Debit,
                TransactionType::TotalInternationalDebitChf,
            ),
            "614" => (Direction::Debit, TransactionType::TotalInternationalDebitFf),
            "615" => (
                Direction::Debit,
                TransactionType::TotalFederalReserveBankCommercialBankDebit,
            ),
            "616" => (
                Direction::Debit,
                TransactionType::FederalReserveBankCommercialBankDebit,
            ),
            "617" => (
                Direction::Debit,
                TransactionType::TotalSecuritiesPurchasedChf,
            ),
            "618" => (
                Direction::Debit,
                TransactionType::TotalSecuritiesPurchasedFf,
            ),
            "621" => (Direction::Debit, TransactionType::TotalBrokerDebitsChf),
            "622" => (Direction::Debit, TransactionType::BrokerDebit),
            "623" => (Direction::Debit, TransactionType::TotalBrokerDebitsFf),
            "625" => (Direction::Debit, TransactionType::TotalBrokerDebits),
            "626" => (Direction::Debit, TransactionType::TotalFedFundsPurchased),
            "627" => (Direction::Debit, TransactionType::FedFundsPurchased),
            "628" => (Direction::Debit, TransactionType::TotalCashCenterDebits),
            "629" => (Direction::Debit, TransactionType::CashCenterDebit),
            "630" => (Direction::Debit, TransactionType::TotalDebitAdjustments),
            "631" => (Direction::Debit, TransactionType::DebitAdjustment),
            "632" => (Direction::Debit, TransactionType::TotalTrustDebits),
            "633" => (Direction::Debit, TransactionType::TrustDebit),
            "634" => (Direction::Debit, TransactionType::YtdAdjustmentDebit),
            "640" => (Direction::Debit, TransactionType::TotalEscrowDebits),
            "641" => (Direction::Debit, TransactionType::IndividualEscrowDebit),
            "644" => (Direction::Debit, TransactionType::IndividualBackValueDebit),
            "646" => (Direction::Debit, TransactionType::TransferCalculationDebit),
            "650" => (Direction::Debit, TransactionType::InvestmentsPurchased),
            "651" => (
                Direction::Debit,
                TransactionType::IndividualInvestmentPurchased,
            ),
            "654" => (Direction::Debit, TransactionType::InterestDebit),
            "655" => (
                Direction::Debit,
                TransactionType::TotalInvestmentInterestDebits,
            ),
            "656" => (Direction::Debit, TransactionType::SweepPrincipalBuy),
            "657" => (Direction::Debit, TransactionType::FuturesDebit),
            "658" => (Direction::Debit, TransactionType::PrincipalPaymentsDebit),
            "659" => (Direction::Debit, TransactionType::InterestAdjustmentDebit),
            "661" => (Direction::Debit, TransactionType::AccountAnalysisFee),
            "662" => (
                Direction::Debit,
                TransactionType::CorrespondentCollectionDebit,
            ),
            "663" => (
                Direction::Debit,
                TransactionType::CorrespondentCollectionAdjustment,
            ),
            "664" => (Direction::Debit, TransactionType::LoanParticipation),
            "665" => (Direction::Debit, TransactionType::InterceptDebits),
            "666" => (Direction::Debit, TransactionType::CurrencyAndCoinShipped),
            "667" => (Direction::Debit, TransactionType::FoodStampLetter),
            "668" => (Direction::Debit, TransactionType::FoodStampAdjustment),
            "669" => (Direction::Debit, TransactionType::ClearingSettlementDebit),
            "670" => (Direction::Debit, TransactionType::TotalBackValueDebits),
            "672" => (Direction::Debit, TransactionType::BackValueAdjustment),
            "673" => (Direction::Debit, TransactionType::CustomerPayroll),
            "674" => (Direction::Debit, TransactionType::FrbStatementRecap),
            "676" => (
                Direction::Debit,
                TransactionType::SavingsBondLetterOrAdjustment,
            ),
            "677" => (Direction::Debit, TransactionType::TreasuryTaxAndLoanDebit),
            "678" => (Direction::Debit, TransactionType::TransferOfTreasuryDebit),
            "679" => (
                Direction::Debit,
                TransactionType::FrbGovernmentChecksCashLetterDebit,
            ),
            "681" => (
                Direction::Debit,
                TransactionType::FrbGovernmentCheckAdjustment,
            ),
            "682" => (Direction::Debit, TransactionType::FrbPostalMoneyOrderDebit),
            "683" => (
                Direction::Debit,
                TransactionType::FrbPostalMoneyOrderAdjustment,
            ),
            "684" => (
                Direction::Debit,
                TransactionType::FrbCashLetterAutoChargeDebit,
            ),
            "685" => (Direction::Debit, TransactionType::TotalUniversalDebits),
            "686" => (
                Direction::Debit,
                TransactionType::FrbCashLetterAutoChargeAdjustment,
            ),
            "687" => (
                Direction::Debit,
                TransactionType::FrbFineSortCashLetterDebit,
            ),
            "688" => (Direction::Debit, TransactionType::FrbFineSortAdjustment),
            "689" => (Direction::Debit, TransactionType::FrbFreightPaymentDebits),
            "690" => (Direction::Debit, TransactionType::TotalMiscellaneousDebits),
            "691" => (Direction::Debit, TransactionType::UniversalDebit),
            "692" => (Direction::Debit, TransactionType::FreightPaymentDebit),
            "693" => (Direction::Debit, TransactionType::ItemizedDebitOver10000),
            "694" => (Direction::Debit, TransactionType::DepositReversal),
            "695" => (Direction::Debit, TransactionType::DepositCorrectionDebit),
            "696" => (Direction::Debit, TransactionType::RegularCollectionDebit),
            "697" => (Direction::Debit, TransactionType::CumulativeDebits),
            "698" => (Direction::Debit, TransactionType::MiscellaneousFees),
            "699" => (Direction::Debit, TransactionType::MiscellaneousDebit),
            "720" => (Direction::Credit, TransactionType::TotalLoanPayment),
            "721" => (Direction::Credit, TransactionType::AmountAppliedToInterest),
            "722" => (Direction::Credit, TransactionType::AmountAppliedToPrincipal),
            "723" => (Direction::Credit, TransactionType::AmountAppliedToEscrow),
            "724" => (
                Direction::Credit,
                TransactionType::AmountAppliedToLateCharges,
            ),
            "725" => (Direction::Credit, TransactionType::AmountAppliedToBuydown),
            "726" => (Direction::Credit, TransactionType::AmountAppliedToMiscFees),
            "727" => (
                Direction::Credit,
                TransactionType::AmountAppliedToDeferredInterestDetail,
            ),
            "728" => (
                Direction::Credit,
                TransactionType::AmountAppliedToServiceCharge,
            ),
            "760" => (Direction::Debit, TransactionType::LoanDisbursement),

            &_ => (Direction::Unknown, TransactionType::Unknown),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Continuation {
    text: String,
}

impl Continuation {
    fn parse(line: String) -> Continuation {
        if let Some((_, text)) = line.split_once(",") {
            Continuation {
                text: text.to_string(),
            }
        } else {
            Continuation {
                text: "".to_string(),
            }
        }
    }
}
