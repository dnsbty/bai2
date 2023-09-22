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

fn parse_date(string: &str) -> Option<NaiveDate> {
    let date = string.trim().replace("/", "");
    let maybe_date = NaiveDate::parse_from_str(&date, "%y%m%d");
    match maybe_date {
        Ok(d) => Some(d),
        Err(_) => None,
    }
}

fn parse_time(string: &str) -> Option<NaiveTime> {
    let time = string.trim().replace("/", "");
    let maybe_time = NaiveTime::parse_from_str(&time, "%H%M");
    match maybe_time {
        Ok(t) => Some(t),
        Err(_) => None,
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
    pub creation_time: Option<NaiveTime>,
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
    as_of_date_modifier: String,
    as_of_time: Option<NaiveTime>,
    currency_code: String,
    receiver: String,
    sender: String,
    status: String,
}

impl GroupHeader {
    fn parse(line: String) -> GroupHeader {
        let fields = line.split(",").collect::<Vec<&str>>();

        GroupHeader {
            as_of_date: parse_date(fields[4]),
            as_of_date_modifier: parse_string(fields[7]),
            as_of_time: parse_time(fields[5]),
            currency_code: parse_string(fields[6]),
            receiver: parse_string(fields[1]),
            sender: parse_string(fields[2]),
            status: parse_string(fields[3]),
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
        match value.trim().replace("/", "").as_str() {
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
            currency_code: parse_string(fields[2]),
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
    funds_type: Option<FundsType>,
    text: String,
    transaction_type: Option<Direction>,
}

impl Transaction {
    fn parse(line: String) -> Transaction {
        let fields = line.split(",").collect::<Vec<&str>>();
        debug!("Parsing transaction: {:?}", fields);

        let mut transaction = Transaction {
            amount: parse_int(fields[2]),
            bank_reference_number: parse_string(fields[4]),
            continuations: Vec::new(),
            customer_reference_number: parse_string(fields[5]),
            funds_type: FundsType::parse(fields[3]),
            text: "".to_string(),
            transaction_type: Direction::parse(fields[1]),
        };

        if fields.len() > 6 {
            transaction.text = parse_string(fields[6]);
        }

        return transaction;
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
enum Direction {
    Credit(TransactionType),
    Debit(TransactionType),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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

impl Direction {
    fn parse(type_code: &str) -> Option<Direction> {
        match type_code.trim().replace("/", "").as_str() {
            "100" => Some(Direction::Credit(TransactionType::TotalCredits)),
            "101" => Some(Direction::Credit(TransactionType::TotalCreditAmountMtd)),
            "105" => Some(Direction::Credit(TransactionType::CreditsNotDetailed)),
            "106" => Some(Direction::Credit(TransactionType::DepositsSubjectToFloat)),
            "107" => Some(Direction::Credit(
                TransactionType::TotalAdjustmentCreditsYtd,
            )),
            "108" => Some(Direction::Credit(TransactionType::Credit)),
            "109" => Some(Direction::Credit(
                TransactionType::CurrentDayTotalLockboxDeposits,
            )),
            "110" => Some(Direction::Credit(TransactionType::TotalLockboxDeposits)),
            "115" => Some(Direction::Credit(TransactionType::LockboxDeposit)),
            "116" => Some(Direction::Credit(TransactionType::ItemInLockboxDeposit)),
            "118" => Some(Direction::Credit(TransactionType::LockboxAdjustmentCredit)),
            "120" => Some(Direction::Credit(TransactionType::EdiTransactionCredit)),
            "121" => Some(Direction::Credit(TransactionType::EdiTransactionCredit)),
            "122" => Some(Direction::Credit(TransactionType::EdibanxCreditReceived)),
            "123" => Some(Direction::Credit(TransactionType::EdibanxCreditReturn)),
            "130" => Some(Direction::Credit(
                TransactionType::TotalConcentrationCredits,
            )),
            "131" => Some(Direction::Credit(TransactionType::TotalDtcCredits)),
            "135" => Some(Direction::Credit(TransactionType::DtcConcentrationCredit)),
            "136" => Some(Direction::Credit(TransactionType::ItemInDtcDeposit)),
            "140" => Some(Direction::Credit(TransactionType::TotalAchCredits)),
            "142" => Some(Direction::Credit(TransactionType::AchCreditReceived)),
            "143" => Some(Direction::Credit(TransactionType::ItemInAchDeposit)),
            "145" => Some(Direction::Credit(TransactionType::AchConcentrationCredit)),
            "146" => Some(Direction::Credit(TransactionType::TotalBankCardDeposits)),
            "147" => Some(Direction::Credit(
                TransactionType::IndividualBankCardDeposit,
            )),
            "150" => Some(Direction::Credit(
                TransactionType::TotalPreauthorizedPaymentCredits,
            )),
            "155" => Some(Direction::Credit(TransactionType::PreauthorizedDraftCredit)),
            "156" => Some(Direction::Credit(TransactionType::ItemInPacDeposit)),
            "160" => Some(Direction::Credit(
                TransactionType::TotalAchDisbursingFundingCredits,
            )),
            "162" => Some(Direction::Credit(
                TransactionType::CorporateTradePaymentSettlement,
            )),
            "163" => Some(Direction::Credit(
                TransactionType::CorporateTradePaymentCredits,
            )),
            "164" => Some(Direction::Credit(
                TransactionType::CorporateTradePaymentCredit,
            )),
            "165" => Some(Direction::Credit(TransactionType::PreauthorizedAchCredit)),
            "166" => Some(Direction::Credit(TransactionType::AchSettlement)),
            "167" => Some(Direction::Credit(TransactionType::AchSettlementCredits)),
            "168" => Some(Direction::Credit(
                TransactionType::AchReturnItemOrAdjustmentSettlement,
            )),
            "169" => Some(Direction::Credit(TransactionType::MiscellaneousAchCredit)),
            "170" => Some(Direction::Credit(TransactionType::TotalOtherCheckDeposits)),
            "171" => Some(Direction::Credit(TransactionType::IndividualLoanDeposit)),
            "172" => Some(Direction::Credit(TransactionType::DepositCorrection)),
            "173" => Some(Direction::Credit(TransactionType::BankPreparedDeposit)),
            "174" => Some(Direction::Credit(TransactionType::OtherDeposit)),
            "175" => Some(Direction::Credit(TransactionType::CheckDepositPackage)),
            "176" => Some(Direction::Credit(TransactionType::RePresentedCheckDeposit)),
            "178" => Some(Direction::Credit(TransactionType::ListPostCredits)),
            "180" => Some(Direction::Credit(TransactionType::TotalLoanProceeds)),
            "182" => Some(Direction::Credit(
                TransactionType::TotalBankPreparedDeposits,
            )),
            "184" => Some(Direction::Credit(TransactionType::DraftDeposit)),
            "185" => Some(Direction::Credit(
                TransactionType::TotalMiscellaneousDeposits,
            )),
            "186" => Some(Direction::Credit(TransactionType::TotalCashLetterCredits)),
            "187" => Some(Direction::Credit(TransactionType::CashLetterCredit)),
            "188" => Some(Direction::Credit(
                TransactionType::TotalCashLetterAdjustments,
            )),
            "189" => Some(Direction::Credit(TransactionType::CashLetterAdjustment)),
            "190" => Some(Direction::Credit(
                TransactionType::TotalIncomingMoneyTransfers,
            )),
            "191" => Some(Direction::Credit(
                TransactionType::IndividualIncomingInternalMoneyTransfer,
            )),
            "195" => Some(Direction::Credit(TransactionType::IncomingMoneyTransfer)),
            "196" => Some(Direction::Credit(TransactionType::MoneyTransferAdjustment)),
            "198" => Some(Direction::Credit(TransactionType::Compensation)),
            "200" => Some(Direction::Credit(
                TransactionType::TotalAutomaticTransferCredits,
            )),
            "201" => Some(Direction::Credit(
                TransactionType::IndividualAutomaticTransferCredit,
            )),
            "202" => Some(Direction::Credit(TransactionType::BondOperationsCredit)),
            "205" => Some(Direction::Credit(TransactionType::TotalBookTransferCredits)),
            "206" => Some(Direction::Credit(TransactionType::BookTransferCredit)),
            "207" => Some(Direction::Credit(
                TransactionType::TotalInternationalMoneyTransferCredits,
            )),
            "208" => Some(Direction::Credit(
                TransactionType::IndividualInternationalMoneyTransferCredit,
            )),
            "210" => Some(Direction::Credit(
                TransactionType::TotalInternationalCredits,
            )),
            "212" => Some(Direction::Credit(TransactionType::ForeignLetterOfCredit)),
            "213" => Some(Direction::Credit(TransactionType::LetterOfCredit)),
            "214" => Some(Direction::Credit(TransactionType::ForeignExchangeOfCredit)),
            "215" => Some(Direction::Credit(TransactionType::TotalLettersOfCredit)),
            "216" => Some(Direction::Credit(TransactionType::ForeignRemittanceCredit)),
            "218" => Some(Direction::Credit(TransactionType::ForeignCollectionCredit)),
            "221" => Some(Direction::Credit(TransactionType::ForeignCheckPurchase)),
            "222" => Some(Direction::Credit(TransactionType::ForeignChecksDeposited)),
            "224" => Some(Direction::Credit(TransactionType::Commission)),
            "226" => Some(Direction::Credit(
                TransactionType::InternationalMoneyMarketTrading,
            )),
            "227" => Some(Direction::Credit(TransactionType::StandingOrder)),
            "229" => Some(Direction::Credit(
                TransactionType::MiscellaneousInternationalCredit,
            )),
            "230" => Some(Direction::Credit(TransactionType::TotalSecurityCredits)),
            "231" => Some(Direction::Credit(TransactionType::TotalCollectionCredits)),
            "232" => Some(Direction::Credit(TransactionType::SaleOfDebtSecurity)),
            "233" => Some(Direction::Credit(TransactionType::SecuritiesSold)),
            "234" => Some(Direction::Credit(TransactionType::SaleOfEquitySecurity)),
            "235" => Some(Direction::Credit(
                TransactionType::MaturedReverseRepurchaseOrder,
            )),
            "236" => Some(Direction::Credit(TransactionType::MaturityOfDebtSecurity)),
            "237" => Some(Direction::Credit(
                TransactionType::IndividualCollectionCredit,
            )),
            "238" => Some(Direction::Credit(TransactionType::CollectionOfDividends)),
            "239" => Some(Direction::Credit(
                TransactionType::TotalBankersAcceptanceCredits,
            )),
            "240" => Some(Direction::Credit(TransactionType::CouponCollectionsBanks)),
            "241" => Some(Direction::Credit(TransactionType::BankersAcceptances)),
            "242" => Some(Direction::Credit(
                TransactionType::CollectionOfInterestIncome,
            )),
            "243" => Some(Direction::Credit(TransactionType::MaturedFedFundsPurchased)),
            "244" => Some(Direction::Credit(
                TransactionType::InterestMaturedPrincipalPayment,
            )),
            "245" => Some(Direction::Credit(TransactionType::MonthlyDividends)),
            "246" => Some(Direction::Credit(TransactionType::CommercialPaper)),
            "247" => Some(Direction::Credit(TransactionType::CapitalChange)),
            "248" => Some(Direction::Credit(
                TransactionType::SavingsBondsSalesAdjustment,
            )),
            "249" => Some(Direction::Credit(
                TransactionType::MiscellaneousSecurityCredit,
            )),
            "250" => Some(Direction::Credit(
                TransactionType::TotalChecksPostedAndReturned,
            )),
            "251" => Some(Direction::Credit(TransactionType::TotalDebitReversals)),
            "252" => Some(Direction::Credit(TransactionType::DebitReversal)),
            "254" => Some(Direction::Credit(
                TransactionType::PostingErrorCorrectionCredit,
            )),
            "255" => Some(Direction::Credit(TransactionType::CheckPostedAndReturned)),
            "256" => Some(Direction::Credit(TransactionType::TotalAchReturnItems)),
            "257" => Some(Direction::Credit(TransactionType::IndividualAchReturnItem)),
            "258" => Some(Direction::Credit(TransactionType::AchReversalCredit)),
            "260" => Some(Direction::Credit(TransactionType::TotalRejectedCredits)),
            "261" => Some(Direction::Credit(TransactionType::IndividualRejectedCredit)),
            "263" => Some(Direction::Credit(TransactionType::Overdraft)),
            "266" => Some(Direction::Credit(TransactionType::ReturnItem)),
            "268" => Some(Direction::Credit(TransactionType::ReturnItemAdjustment)),
            "270" => Some(Direction::Credit(TransactionType::TotalZbaCredits)),
            "271" => Some(Direction::Credit(TransactionType::NetZeroBalanceAmount)),
            "274" => Some(Direction::Credit(
                TransactionType::CumulativeZbaOrDisbursementCredits,
            )),
            "275" => Some(Direction::Credit(TransactionType::ZbaCredit)),
            "276" => Some(Direction::Credit(TransactionType::ZbaFloatAdjustment)),
            "277" => Some(Direction::Credit(TransactionType::ZbaCreditTransfer)),
            "278" => Some(Direction::Credit(TransactionType::ZbaCreditAdjustment)),
            "280" => Some(Direction::Credit(
                TransactionType::TotalControlledDisbursingCredits,
            )),
            "281" => Some(Direction::Credit(
                TransactionType::IndividualControlledDisbursingCredit,
            )),
            "285" => Some(Direction::Credit(
                TransactionType::TotalDtcDisbursingCredits,
            )),
            "286" => Some(Direction::Credit(
                TransactionType::IndividualDtcDisbursingCredit,
            )),
            "294" => Some(Direction::Credit(TransactionType::TotalAtmCredits)),
            "295" => Some(Direction::Credit(TransactionType::AtmCredit)),
            "301" => Some(Direction::Credit(TransactionType::CommercialDeposit)),
            "302" => Some(Direction::Credit(TransactionType::CorrespondentBankDeposit)),
            "303" => Some(Direction::Credit(TransactionType::TotalWireTransfersInFF)),
            "304" => Some(Direction::Credit(TransactionType::TotalWireTransfersInCHF)),
            "305" => Some(Direction::Credit(TransactionType::TotalFedFundsSold)),
            "306" => Some(Direction::Credit(TransactionType::FedFundsSold)),
            "307" => Some(Direction::Credit(TransactionType::TotalTrustCredits)),
            "308" => Some(Direction::Credit(TransactionType::TrustCredit)),
            "309" => Some(Direction::Credit(TransactionType::TotalValueDatedFunds)),
            "310" => Some(Direction::Credit(TransactionType::TotalCommercialDeposits)),
            "315" => Some(Direction::Credit(
                TransactionType::TotalInternationalCreditsFf,
            )),
            "316" => Some(Direction::Credit(
                TransactionType::TotalInternationalCreditsChf,
            )),
            "318" => Some(Direction::Credit(
                TransactionType::TotalForeignCheckPurchased,
            )),
            "319" => Some(Direction::Credit(TransactionType::LateDeposit)),
            "320" => Some(Direction::Credit(TransactionType::TotalSecuritiesSoldFf)),
            "321" => Some(Direction::Credit(TransactionType::TotalSecuritiesSoldChf)),
            "324" => Some(Direction::Credit(TransactionType::TotalSecuritiesMaturedFf)),
            "325" => Some(Direction::Credit(
                TransactionType::TotalSecuritiesMaturedChf,
            )),
            "326" => Some(Direction::Credit(TransactionType::TotalSecuritiesInterest)),
            "327" => Some(Direction::Credit(TransactionType::TotalSecuritiesMatured)),
            "328" => Some(Direction::Credit(
                TransactionType::TotalSecuritiesInterestFf,
            )),
            "329" => Some(Direction::Credit(
                TransactionType::TotalSecuritiesInterestChf,
            )),
            "330" => Some(Direction::Credit(TransactionType::TotalEscrowCredits)),
            "331" => Some(Direction::Credit(TransactionType::IndividualEscrowCredit)),
            "332" => Some(Direction::Credit(
                TransactionType::TotalMiscellaneousSecuritiesCreditsFf,
            )),
            "336" => Some(Direction::Credit(
                TransactionType::TotalMiscellaneousSecuritiesCreditsChf,
            )),
            "338" => Some(Direction::Credit(TransactionType::TotalSecuritiesSold)),
            "340" => Some(Direction::Credit(TransactionType::TotalBrokerDeposits)),
            "341" => Some(Direction::Credit(TransactionType::TotalBrokerDepositsFf)),
            "342" => Some(Direction::Credit(TransactionType::BrokerDeposit)),
            "343" => Some(Direction::Credit(TransactionType::TotalBrokerDepositsChf)),
            "344" => Some(Direction::Credit(
                TransactionType::IndividualBackValueCredit,
            )),
            "345" => Some(Direction::Credit(TransactionType::ItemInBrokersDeposit)),
            "346" => Some(Direction::Credit(TransactionType::SweepInterestIncome)),
            "347" => Some(Direction::Credit(TransactionType::SweepPrincipalSell)),
            "348" => Some(Direction::Credit(TransactionType::FuturesCredit)),
            "349" => Some(Direction::Credit(TransactionType::PrincipalPaymentsCredit)),
            "350" => Some(Direction::Credit(TransactionType::InvestmentSold)),
            "351" => Some(Direction::Credit(TransactionType::IndividualInvestmentSold)),
            "352" => Some(Direction::Credit(TransactionType::TotalCashCenterCredits)),
            "353" => Some(Direction::Credit(TransactionType::CashCenterCredit)),
            "354" => Some(Direction::Credit(TransactionType::InterestCredit)),
            "355" => Some(Direction::Credit(TransactionType::InvestmentInterest)),
            "356" => Some(Direction::Credit(TransactionType::TotalCreditAdjustment)),
            "357" => Some(Direction::Credit(TransactionType::CreditAdjustment)),
            "358" => Some(Direction::Credit(TransactionType::YtdAdjustmentCredit)),
            "359" => Some(Direction::Credit(TransactionType::InterestAdjustmentCredit)),
            "360" => Some(Direction::Credit(
                TransactionType::TotalCreditsLessWireTransferAndReturnedChecks,
            )),
            "361" => Some(Direction::Credit(
                TransactionType::GrandTotalCreditsLessGrandTotalDebits,
            )),
            "362" => Some(Direction::Credit(TransactionType::CorrespondentCollection)),
            "363" => Some(Direction::Credit(
                TransactionType::CorrespondentCollectionAdjustment,
            )),
            "364" => Some(Direction::Credit(TransactionType::LoanParticipation)),
            "366" => Some(Direction::Credit(TransactionType::CurrencyAndCoinDeposited)),
            "367" => Some(Direction::Credit(TransactionType::FoodStampLetter)),
            "368" => Some(Direction::Credit(TransactionType::FoodStampAdjustment)),
            "369" => Some(Direction::Credit(TransactionType::ClearingSettlementCredit)),
            "370" => Some(Direction::Credit(TransactionType::TotalBackValueCredits)),
            "372" => Some(Direction::Credit(TransactionType::BackValueAdjustment)),
            "373" => Some(Direction::Credit(TransactionType::CustomerPayroll)),
            "374" => Some(Direction::Credit(TransactionType::FrbStatementRecap)),
            "376" => Some(Direction::Credit(
                TransactionType::SavingsBondLetterOrAdjustment,
            )),
            "377" => Some(Direction::Credit(TransactionType::TreasuryTaxAndLoanCredit)),
            "378" => Some(Direction::Credit(TransactionType::TransferOfTreasuryCredit)),
            "379" => Some(Direction::Credit(
                TransactionType::FrbGovernmentChecksCashLetterCredit,
            )),
            "381" => Some(Direction::Credit(
                TransactionType::FrbGovernmentCheckAdjustment,
            )),
            "382" => Some(Direction::Credit(
                TransactionType::FrbPostalMoneyOrderCredit,
            )),
            "383" => Some(Direction::Credit(
                TransactionType::FrbPostalMoneyOrderAdjustment,
            )),
            "384" => Some(Direction::Credit(
                TransactionType::FrbCashLetterAutoChargeCredit,
            )),
            "385" => Some(Direction::Credit(TransactionType::TotalUniversalCredits)),
            "386" => Some(Direction::Credit(
                TransactionType::FrbCashLetterAutoChargeAdjustment,
            )),
            "387" => Some(Direction::Credit(
                TransactionType::FrbFineSortCashLetterCredit,
            )),
            "388" => Some(Direction::Credit(TransactionType::FrbFineSortAdjustment)),
            "389" => Some(Direction::Credit(
                TransactionType::TotalFreightPaymentCredits,
            )),
            "390" => Some(Direction::Credit(
                TransactionType::TotalMiscellaneousCredits,
            )),
            "391" => Some(Direction::Credit(TransactionType::UniversalCredit)),
            "392" => Some(Direction::Credit(TransactionType::FreightPaymentCredit)),
            "393" => Some(Direction::Credit(TransactionType::ItemizedCreditOver10000)),
            "394" => Some(Direction::Credit(TransactionType::CumulativeCredits)),
            "395" => Some(Direction::Credit(TransactionType::CheckReversal)),
            "397" => Some(Direction::Credit(TransactionType::FloatAdjustment)),
            "398" => Some(Direction::Credit(TransactionType::MiscellaneousFeeRefund)),
            "399" => Some(Direction::Credit(TransactionType::MiscellaneousCredit)),
            "401" => Some(Direction::Debit(TransactionType::TotalDebitAmountMtd)),
            "403" => Some(Direction::Debit(TransactionType::TodaysTotalDebits)),
            "405" => Some(Direction::Debit(
                TransactionType::TotalDebitLessWireTransfersAndChargeBacks,
            )),
            "406" => Some(Direction::Debit(TransactionType::DebitsNotDetailed)),
            "408" => Some(Direction::Debit(TransactionType::FloatAdjustment)),
            "409" => Some(Direction::Debit(TransactionType::DebitAnyType)),
            "410" => Some(Direction::Debit(TransactionType::TotalYtdAdjustment)),
            "412" => Some(Direction::Debit(
                TransactionType::TotalDebitsExcludingReturnedItems,
            )),
            "415" => Some(Direction::Debit(TransactionType::LockboxDebit)),
            "416" => Some(Direction::Debit(TransactionType::TotalLockboxDebits)),
            "420" => Some(Direction::Debit(TransactionType::EdiTransactionDebits)),
            "421" => Some(Direction::Debit(TransactionType::EdiTransactionDebit)),
            "422" => Some(Direction::Debit(TransactionType::EdibanxSettlementDebit)),
            "423" => Some(Direction::Debit(TransactionType::EdibanxReturnItemDebit)),
            "430" => Some(Direction::Debit(TransactionType::TotalPayableThroughDrafts)),
            "435" => Some(Direction::Debit(TransactionType::PayableThroughDraft)),
            "445" => Some(Direction::Debit(TransactionType::AchConcentrationDebit)),
            "446" => Some(Direction::Debit(
                TransactionType::TotalAchDisbursementFundingDebits,
            )),
            "447" => Some(Direction::Debit(
                TransactionType::AchDisbursementFundingDebit,
            )),
            "450" => Some(Direction::Debit(TransactionType::TotalAchDebits)),
            "451" => Some(Direction::Debit(TransactionType::AchDebitReceived)),
            "452" => Some(Direction::Debit(
                TransactionType::ItemInAchDisbursementOrDebit,
            )),
            "455" => Some(Direction::Debit(TransactionType::PreauthorizedAchDebit)),
            "462" => Some(Direction::Debit(
                TransactionType::AccountHolderInitiatedAchDebit,
            )),
            "463" => Some(Direction::Debit(
                TransactionType::CorporateTradePaymentDebits,
            )),
            "464" => Some(Direction::Debit(
                TransactionType::CorporateTradePaymentDebit,
            )),
            "465" => Some(Direction::Debit(
                TransactionType::CorporateTradePaymentSettlement,
            )),
            "466" => Some(Direction::Debit(TransactionType::AchSettlement)),
            "467" => Some(Direction::Debit(TransactionType::AchSettlementDebits)),
            "468" => Some(Direction::Debit(
                TransactionType::AchReturnItemOrAdjustmentSettlement,
            )),
            "469" => Some(Direction::Debit(TransactionType::MiscellaneousAchDebit)),
            "470" => Some(Direction::Debit(TransactionType::TotalCheckPaid)),
            "471" => Some(Direction::Debit(
                TransactionType::TotalCheckPaidCumulativeMtd,
            )),
            "472" => Some(Direction::Debit(TransactionType::CumulativeChecksPaid)),
            "474" => Some(Direction::Debit(TransactionType::CertifiedCheckDebit)),
            "475" => Some(Direction::Debit(TransactionType::CheckPaid)),
            "476" => Some(Direction::Debit(
                TransactionType::FederalReserveBankLetterDebit,
            )),
            "477" => Some(Direction::Debit(TransactionType::BankOriginatedDebit)),
            "478" => Some(Direction::Debit(TransactionType::ListPostDebits)),
            "479" => Some(Direction::Debit(TransactionType::ListPostDebit)),
            "480" => Some(Direction::Debit(TransactionType::TotalLoanPayments)),
            "481" => Some(Direction::Debit(TransactionType::IndividualLoanPayment)),
            "482" => Some(Direction::Debit(TransactionType::TotalBankOriginatedDebits)),
            "484" => Some(Direction::Debit(TransactionType::Draft)),
            "485" => Some(Direction::Debit(TransactionType::DtcDebit)),
            "486" => Some(Direction::Debit(TransactionType::TotalCashLetterDebits)),
            "487" => Some(Direction::Debit(TransactionType::CashLetterDebit)),
            "489" => Some(Direction::Debit(TransactionType::CashLetterAdjustment)),
            "490" => Some(Direction::Debit(
                TransactionType::TotalOutgoingMoneyTransfers,
            )),
            "491" => Some(Direction::Debit(
                TransactionType::IndividualOutgoingInternalMoneyTransfer,
            )),
            "493" => Some(Direction::Debit(
                TransactionType::CustomerTerminalInitiatedMoneyTransfer,
            )),
            "495" => Some(Direction::Debit(TransactionType::OutgoingMoneyTransfer)),
            "496" => Some(Direction::Debit(TransactionType::MoneyTransferAdjustment)),
            "498" => Some(Direction::Debit(TransactionType::Compensation)),
            "500" => Some(Direction::Debit(
                TransactionType::TotalAutomaticTransferDebits,
            )),
            "501" => Some(Direction::Debit(
                TransactionType::IndividualAutomaticTransferDebit,
            )),
            "502" => Some(Direction::Debit(TransactionType::BondOperationsDebit)),
            "505" => Some(Direction::Debit(TransactionType::TotalBookTransferDebits)),
            "506" => Some(Direction::Debit(TransactionType::BookTransferDebit)),
            "507" => Some(Direction::Debit(
                TransactionType::TotalInternationalMoneyTransferDebits,
            )),
            "508" => Some(Direction::Debit(
                TransactionType::IndividualInternationalMoneyTransferDebits,
            )),
            "510" => Some(Direction::Debit(TransactionType::TotalInternationalDebits)),
            "512" => Some(Direction::Debit(TransactionType::LetterOfCreditDebit)),
            "513" => Some(Direction::Debit(TransactionType::LetterOfCredit)),
            "514" => Some(Direction::Debit(TransactionType::ForeignExchangeDebit)),
            "515" => Some(Direction::Debit(TransactionType::TotalLettersOfCredit)),
            "516" => Some(Direction::Debit(TransactionType::ForeignRemittanceDebit)),
            "518" => Some(Direction::Debit(TransactionType::ForeignCollectionDebit)),
            "522" => Some(Direction::Debit(TransactionType::ForeignChecksPaid)),
            "524" => Some(Direction::Debit(TransactionType::Commission)),
            "526" => Some(Direction::Debit(
                TransactionType::InternationalMoneyMarketTrading,
            )),
            "527" => Some(Direction::Debit(TransactionType::StandingOrder)),
            "529" => Some(Direction::Debit(
                TransactionType::MiscellaneousInternationalDebit,
            )),
            "530" => Some(Direction::Debit(TransactionType::TotalSecurityDebits)),
            "531" => Some(Direction::Debit(TransactionType::SecuritiesPurchased)),
            "532" => Some(Direction::Debit(
                TransactionType::TotalAmountOfSecuritiesPurchased,
            )),
            "533" => Some(Direction::Debit(TransactionType::SecurityCollectionDebit)),
            "534" => Some(Direction::Debit(
                TransactionType::TotalMiscellaneousSecuritiesDbFf,
            )),
            "535" => Some(Direction::Debit(
                TransactionType::PurchaseOfEquitySecurities,
            )),
            "536" => Some(Direction::Debit(
                TransactionType::TotalMiscellaneousSecuritiesDebitChf,
            )),
            "537" => Some(Direction::Debit(TransactionType::TotalCollectionDebit)),
            "538" => Some(Direction::Debit(TransactionType::MaturedRepurchaseOrder)),
            "539" => Some(Direction::Debit(
                TransactionType::TotalBankersAcceptancesDebit,
            )),
            "540" => Some(Direction::Debit(TransactionType::CouponCollectionDebit)),
            "541" => Some(Direction::Debit(TransactionType::BankersAcceptances)),
            "542" => Some(Direction::Debit(TransactionType::PurchaseOfDebtSecurities)),
            "543" => Some(Direction::Debit(TransactionType::DomesticCollection)),
            "544" => Some(Direction::Debit(
                TransactionType::InterestMaturedPrincipalPayment,
            )),
            "546" => Some(Direction::Debit(TransactionType::CommercialPaper)),
            "547" => Some(Direction::Debit(TransactionType::CapitalChange)),
            "548" => Some(Direction::Debit(
                TransactionType::SavingsBondsSalesAdjustment,
            )),
            "549" => Some(Direction::Debit(
                TransactionType::MiscellaneousSecurityDebit,
            )),
            "550" => Some(Direction::Debit(
                TransactionType::TotalDepositedItemsReturned,
            )),
            "551" => Some(Direction::Debit(TransactionType::TotalCreditReversals)),
            "552" => Some(Direction::Debit(TransactionType::CreditReversal)),
            "554" => Some(Direction::Debit(
                TransactionType::PostingErrorCorrectionDebit,
            )),
            "555" => Some(Direction::Debit(TransactionType::DepositedItemReturned)),
            "556" => Some(Direction::Debit(TransactionType::TotalAchReturnItems)),
            "557" => Some(Direction::Debit(TransactionType::IndividualAchReturnItem)),
            "558" => Some(Direction::Debit(TransactionType::AchReversalDebit)),
            "560" => Some(Direction::Debit(TransactionType::TotalRejectedDebits)),
            "561" => Some(Direction::Debit(TransactionType::IndividualRejectedDebit)),
            "563" => Some(Direction::Debit(TransactionType::Overdraft)),
            "564" => Some(Direction::Debit(TransactionType::OverdraftFee)),
            "566" => Some(Direction::Debit(TransactionType::ReturnItem)),
            "567" => Some(Direction::Debit(TransactionType::ReturnItemFee)),
            "568" => Some(Direction::Debit(TransactionType::ReturnItemAdjustment)),
            "570" => Some(Direction::Debit(TransactionType::TotalZbaDebits)),
            "574" => Some(Direction::Debit(TransactionType::CumulativeZbaDebits)),
            "575" => Some(Direction::Debit(TransactionType::ZbaDebit)),
            "577" => Some(Direction::Debit(TransactionType::ZbaDebitTransfer)),
            "578" => Some(Direction::Debit(TransactionType::ZbaDebitAdjustment)),
            "580" => Some(Direction::Debit(
                TransactionType::TotalControlledDisbursingDebits,
            )),
            "581" => Some(Direction::Debit(
                TransactionType::IndividualControlledDisbursingDebit,
            )),
            "583" => Some(Direction::Debit(
                TransactionType::TotalDisbursingChecksPaidEarlyAmount,
            )),
            "584" => Some(Direction::Debit(
                TransactionType::TotalDisbursingChecksPaidLaterAmount,
            )),
            "585" => Some(Direction::Debit(
                TransactionType::DisbursingFundingRequirement,
            )),
            "586" => Some(Direction::Debit(TransactionType::FrbPresentmentEstimate)),
            "587" => Some(Direction::Debit(
                TransactionType::LateDebitsAfterNotification,
            )),
            "588" => Some(Direction::Debit(
                TransactionType::TotalDisbursingChecksPaidLastAmount,
            )),
            "590" => Some(Direction::Debit(TransactionType::TotalDtcDebits)),
            "594" => Some(Direction::Debit(TransactionType::TotalAtmDebits)),
            "595" => Some(Direction::Debit(TransactionType::AtmDebit)),
            "596" => Some(Direction::Debit(TransactionType::TotalAprDebits)),
            "597" => Some(Direction::Debit(TransactionType::ArpDebit)),
            "601" => Some(Direction::Debit(
                TransactionType::EstimatedTotalDisbursement,
            )),
            "602" => Some(Direction::Debit(TransactionType::AdjustedTotalDisbursement)),
            "610" => Some(Direction::Debit(TransactionType::TotalFundsRequired)),
            "611" => Some(Direction::Debit(TransactionType::TotalWireTransfersOutChf)),
            "612" => Some(Direction::Debit(TransactionType::TotalWireTransfersOutFf)),
            "613" => Some(Direction::Debit(
                TransactionType::TotalInternationalDebitChf,
            )),
            "614" => Some(Direction::Debit(TransactionType::TotalInternationalDebitFf)),
            "615" => Some(Direction::Debit(
                TransactionType::TotalFederalReserveBankCommercialBankDebit,
            )),
            "616" => Some(Direction::Debit(
                TransactionType::FederalReserveBankCommercialBankDebit,
            )),
            "617" => Some(Direction::Debit(
                TransactionType::TotalSecuritiesPurchasedChf,
            )),
            "618" => Some(Direction::Debit(
                TransactionType::TotalSecuritiesPurchasedFf,
            )),
            "621" => Some(Direction::Debit(TransactionType::TotalBrokerDebitsChf)),
            "622" => Some(Direction::Debit(TransactionType::BrokerDebit)),
            "623" => Some(Direction::Debit(TransactionType::TotalBrokerDebitsFf)),
            "625" => Some(Direction::Debit(TransactionType::TotalBrokerDebits)),
            "626" => Some(Direction::Debit(TransactionType::TotalFedFundsPurchased)),
            "627" => Some(Direction::Debit(TransactionType::FedFundsPurchased)),
            "628" => Some(Direction::Debit(TransactionType::TotalCashCenterDebits)),
            "629" => Some(Direction::Debit(TransactionType::CashCenterDebit)),
            "630" => Some(Direction::Debit(TransactionType::TotalDebitAdjustments)),
            "631" => Some(Direction::Debit(TransactionType::DebitAdjustment)),
            "632" => Some(Direction::Debit(TransactionType::TotalTrustDebits)),
            "633" => Some(Direction::Debit(TransactionType::TrustDebit)),
            "634" => Some(Direction::Debit(TransactionType::YtdAdjustmentDebit)),
            "640" => Some(Direction::Debit(TransactionType::TotalEscrowDebits)),
            "641" => Some(Direction::Debit(TransactionType::IndividualEscrowDebit)),
            "644" => Some(Direction::Debit(TransactionType::IndividualBackValueDebit)),
            "646" => Some(Direction::Debit(TransactionType::TransferCalculationDebit)),
            "650" => Some(Direction::Debit(TransactionType::InvestmentsPurchased)),
            "651" => Some(Direction::Debit(
                TransactionType::IndividualInvestmentPurchased,
            )),
            "654" => Some(Direction::Debit(TransactionType::InterestDebit)),
            "655" => Some(Direction::Debit(
                TransactionType::TotalInvestmentInterestDebits,
            )),
            "656" => Some(Direction::Debit(TransactionType::SweepPrincipalBuy)),
            "657" => Some(Direction::Debit(TransactionType::FuturesDebit)),
            "658" => Some(Direction::Debit(TransactionType::PrincipalPaymentsDebit)),
            "659" => Some(Direction::Debit(TransactionType::InterestAdjustmentDebit)),
            "661" => Some(Direction::Debit(TransactionType::AccountAnalysisFee)),
            "662" => Some(Direction::Debit(
                TransactionType::CorrespondentCollectionDebit,
            )),
            "663" => Some(Direction::Debit(
                TransactionType::CorrespondentCollectionAdjustment,
            )),
            "664" => Some(Direction::Debit(TransactionType::LoanParticipation)),
            "665" => Some(Direction::Debit(TransactionType::InterceptDebits)),
            "666" => Some(Direction::Debit(TransactionType::CurrencyAndCoinShipped)),
            "667" => Some(Direction::Debit(TransactionType::FoodStampLetter)),
            "668" => Some(Direction::Debit(TransactionType::FoodStampAdjustment)),
            "669" => Some(Direction::Debit(TransactionType::ClearingSettlementDebit)),
            "670" => Some(Direction::Debit(TransactionType::TotalBackValueDebits)),
            "672" => Some(Direction::Debit(TransactionType::BackValueAdjustment)),
            "673" => Some(Direction::Debit(TransactionType::CustomerPayroll)),
            "674" => Some(Direction::Debit(TransactionType::FrbStatementRecap)),
            "676" => Some(Direction::Debit(
                TransactionType::SavingsBondLetterOrAdjustment,
            )),
            "677" => Some(Direction::Debit(TransactionType::TreasuryTaxAndLoanDebit)),
            "678" => Some(Direction::Debit(TransactionType::TransferOfTreasuryDebit)),
            "679" => Some(Direction::Debit(
                TransactionType::FrbGovernmentChecksCashLetterDebit,
            )),
            "681" => Some(Direction::Debit(
                TransactionType::FrbGovernmentCheckAdjustment,
            )),
            "682" => Some(Direction::Debit(TransactionType::FrbPostalMoneyOrderDebit)),
            "683" => Some(Direction::Debit(
                TransactionType::FrbPostalMoneyOrderAdjustment,
            )),
            "684" => Some(Direction::Debit(
                TransactionType::FrbCashLetterAutoChargeDebit,
            )),
            "685" => Some(Direction::Debit(TransactionType::TotalUniversalDebits)),
            "686" => Some(Direction::Debit(
                TransactionType::FrbCashLetterAutoChargeAdjustment,
            )),
            "687" => Some(Direction::Debit(
                TransactionType::FrbFineSortCashLetterDebit,
            )),
            "688" => Some(Direction::Debit(TransactionType::FrbFineSortAdjustment)),
            "689" => Some(Direction::Debit(TransactionType::FrbFreightPaymentDebits)),
            "690" => Some(Direction::Debit(TransactionType::TotalMiscellaneousDebits)),
            "691" => Some(Direction::Debit(TransactionType::UniversalDebit)),
            "692" => Some(Direction::Debit(TransactionType::FreightPaymentDebit)),
            "693" => Some(Direction::Debit(TransactionType::ItemizedDebitOver10000)),
            "694" => Some(Direction::Debit(TransactionType::DepositReversal)),
            "695" => Some(Direction::Debit(TransactionType::DepositCorrectionDebit)),
            "696" => Some(Direction::Debit(TransactionType::RegularCollectionDebit)),
            "697" => Some(Direction::Debit(TransactionType::CumulativeDebits)),
            "698" => Some(Direction::Debit(TransactionType::MiscellaneousFees)),
            "699" => Some(Direction::Debit(TransactionType::MiscellaneousDebit)),
            "720" => Some(Direction::Credit(TransactionType::TotalLoanPayment)),
            "721" => Some(Direction::Credit(TransactionType::AmountAppliedToInterest)),
            "722" => Some(Direction::Credit(TransactionType::AmountAppliedToPrincipal)),
            "723" => Some(Direction::Credit(TransactionType::AmountAppliedToEscrow)),
            "724" => Some(Direction::Credit(
                TransactionType::AmountAppliedToLateCharges,
            )),
            "725" => Some(Direction::Credit(TransactionType::AmountAppliedToBuydown)),
            "726" => Some(Direction::Credit(TransactionType::AmountAppliedToMiscFees)),
            "727" => Some(Direction::Credit(
                TransactionType::AmountAppliedToDeferredInterestDetail,
            )),
            "728" => Some(Direction::Credit(
                TransactionType::AmountAppliedToServiceCharge,
            )),
            "760" => Some(Direction::Debit(TransactionType::LoanDisbursement)),

            &_ => None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Continuation {
    text: String,
}

impl Continuation {
    fn parse(line: String) -> Continuation {
        let fields = line.split(",").collect::<Vec<&str>>();

        Continuation {
            text: parse_string(fields[1]),
        }
    }
}
