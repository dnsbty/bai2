use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;

#[derive(Debug)]
pub enum TransactionType {
    Credit(String, TransactionSubType),
    Debit(String, TransactionSubType),
    Unknown(String, TransactionSubType),
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TransactionSubType {
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
    CorporateTradePaymentDebit,
    CorrespondentCollection,
    CorrespondentCollectionAdjustment,
    CorrespondentCollectionDebit,
    CouponCollectionDebit,
    CouponCollectionsBanks,
    Credit,
    CreditAdjustment,
    CreditReversal,
    CumulativeChecksPaid,
    CumulativeCredits,
    CumulativeDebits,
    CumulativeZbaDebits,
    CumulativeZbaOrDisbursementCredits,
    CurrencyAndCoinDeposited,
    CurrencyAndCoinShipped,
    Custom,
    CustomerPayroll,
    CustomerTerminalInitiatedMoneyTransfer,
    DebitAdjustment,
    DebitAnyType,
    DebitReversal,
    DepositCorrection,
    DepositCorrectionDebit,
    DepositedItemReturned,
    DepositReversal,
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
    FrbGovernmentCheckAdjustment,
    FrbGovernmentChecksCashLetterCredit,
    FrbGovernmentChecksCashLetterDebit,
    FrbPostalMoneyOrderAdjustment,
    FrbPostalMoneyOrderCredit,
    FrbPostalMoneyOrderDebit,
    FrbStatementRecap,
    FreightPaymentCredit,
    FreightPaymentDebit,
    FuturesCredit,
    FuturesDebit,
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
    Info,
    InterestAdjustmentCredit,
    InterestAdjustmentDebit,
    InterestCredit,
    InterestDebit,
    InterestMaturedPrincipalPayment,
    InternationalMoneyMarketTrading,
    ItemInAchDeposit,
    ItemInAchDisbursementOrDebit,
    ItemInBrokersDeposit,
    ItemInDtcDeposit,
    ItemInLockboxDeposit,
    ItemInPacDeposit,
    ItemizedCreditOver10000,
    ItemizedDebitOver10000,
    LetterOfCredit,
    LetterOfCreditDebit,
    ListPostDebit,
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
    pub fn parse(type_code: &str) -> TransactionType {
        let code = type_code.to_string();

        match type_code {
            "108" => TransactionType::Credit(code, TransactionSubType::Credit),
            "115" => TransactionType::Credit(code, TransactionSubType::LockboxDeposit),
            "116" => TransactionType::Credit(code, TransactionSubType::ItemInLockboxDeposit),
            "118" => TransactionType::Credit(code, TransactionSubType::LockboxAdjustmentCredit),
            "121" => TransactionType::Credit(code, TransactionSubType::EdiTransactionCredit),
            "122" => TransactionType::Credit(code, TransactionSubType::EdibanxCreditReceived),
            "123" => TransactionType::Credit(code, TransactionSubType::EdibanxCreditReturn),
            "135" => TransactionType::Credit(code, TransactionSubType::DtcConcentrationCredit),
            "136" => TransactionType::Credit(code, TransactionSubType::ItemInDtcDeposit),
            "142" => TransactionType::Credit(code, TransactionSubType::AchCreditReceived),
            "143" => TransactionType::Credit(code, TransactionSubType::ItemInAchDeposit),
            "145" => TransactionType::Credit(code, TransactionSubType::AchConcentrationCredit),
            "147" => TransactionType::Credit(code, TransactionSubType::IndividualBankCardDeposit),
            "155" => TransactionType::Credit(code, TransactionSubType::PreauthorizedDraftCredit),
            "156" => TransactionType::Credit(code, TransactionSubType::ItemInPacDeposit),
            "164" => TransactionType::Credit(code, TransactionSubType::CorporateTradePaymentCredit),
            "165" => TransactionType::Credit(code, TransactionSubType::PreauthorizedAchCredit),
            "166" => TransactionType::Credit(code, TransactionSubType::AchSettlement),
            "168" => TransactionType::Credit(
                code,
                TransactionSubType::AchReturnItemOrAdjustmentSettlement,
            ),
            "169" => TransactionType::Credit(code, TransactionSubType::MiscellaneousAchCredit),
            "171" => TransactionType::Credit(code, TransactionSubType::IndividualLoanDeposit),
            "172" => TransactionType::Credit(code, TransactionSubType::DepositCorrection),
            "173" => TransactionType::Credit(code, TransactionSubType::BankPreparedDeposit),
            "174" => TransactionType::Credit(code, TransactionSubType::OtherDeposit),
            "175" => TransactionType::Credit(code, TransactionSubType::CheckDepositPackage),
            "176" => TransactionType::Credit(code, TransactionSubType::RePresentedCheckDeposit),
            "184" => TransactionType::Credit(code, TransactionSubType::DraftDeposit),
            "187" => TransactionType::Credit(code, TransactionSubType::CashLetterCredit),
            "189" => TransactionType::Credit(code, TransactionSubType::CashLetterAdjustment),
            "191" => TransactionType::Credit(
                code,
                TransactionSubType::IndividualIncomingInternalMoneyTransfer,
            ),
            "195" => TransactionType::Credit(code, TransactionSubType::IncomingMoneyTransfer),
            "196" => TransactionType::Credit(code, TransactionSubType::MoneyTransferAdjustment),
            "198" => TransactionType::Credit(code, TransactionSubType::Compensation),
            "201" => {
                TransactionType::Credit(code, TransactionSubType::IndividualAutomaticTransferCredit)
            }
            "202" => TransactionType::Credit(code, TransactionSubType::BondOperationsCredit),
            "206" => TransactionType::Credit(code, TransactionSubType::BookTransferCredit),
            "208" => TransactionType::Credit(
                code,
                TransactionSubType::IndividualInternationalMoneyTransferCredit,
            ),
            "212" => TransactionType::Credit(code, TransactionSubType::ForeignLetterOfCredit),
            "213" => TransactionType::Credit(code, TransactionSubType::LetterOfCredit),
            "214" => TransactionType::Credit(code, TransactionSubType::ForeignExchangeOfCredit),
            "216" => TransactionType::Credit(code, TransactionSubType::ForeignRemittanceCredit),
            "218" => TransactionType::Credit(code, TransactionSubType::ForeignCollectionCredit),
            "221" => TransactionType::Credit(code, TransactionSubType::ForeignCheckPurchase),
            "222" => TransactionType::Credit(code, TransactionSubType::ForeignChecksDeposited),
            "224" => TransactionType::Credit(code, TransactionSubType::Commission),
            "226" => {
                TransactionType::Credit(code, TransactionSubType::InternationalMoneyMarketTrading)
            }
            "227" => TransactionType::Credit(code, TransactionSubType::StandingOrder),
            "229" => {
                TransactionType::Credit(code, TransactionSubType::MiscellaneousInternationalCredit)
            }
            "232" => TransactionType::Credit(code, TransactionSubType::SaleOfDebtSecurity),
            "233" => TransactionType::Credit(code, TransactionSubType::SecuritiesSold),
            "234" => TransactionType::Credit(code, TransactionSubType::SaleOfEquitySecurity),
            "235" => {
                TransactionType::Credit(code, TransactionSubType::MaturedReverseRepurchaseOrder)
            }
            "236" => TransactionType::Credit(code, TransactionSubType::MaturityOfDebtSecurity),
            "237" => TransactionType::Credit(code, TransactionSubType::IndividualCollectionCredit),
            "238" => TransactionType::Credit(code, TransactionSubType::CollectionOfDividends),
            "240" => TransactionType::Credit(code, TransactionSubType::CouponCollectionsBanks),
            "241" => TransactionType::Credit(code, TransactionSubType::BankersAcceptances),
            "242" => TransactionType::Credit(code, TransactionSubType::CollectionOfInterestIncome),
            "243" => TransactionType::Credit(code, TransactionSubType::MaturedFedFundsPurchased),
            "244" => {
                TransactionType::Credit(code, TransactionSubType::InterestMaturedPrincipalPayment)
            }
            "246" => TransactionType::Credit(code, TransactionSubType::CommercialPaper),
            "247" => TransactionType::Credit(code, TransactionSubType::CapitalChange),
            "248" => TransactionType::Credit(code, TransactionSubType::SavingsBondsSalesAdjustment),
            "249" => TransactionType::Credit(code, TransactionSubType::MiscellaneousSecurityCredit),
            "252" => TransactionType::Credit(code, TransactionSubType::DebitReversal),
            "254" => {
                TransactionType::Credit(code, TransactionSubType::PostingErrorCorrectionCredit)
            }
            "255" => TransactionType::Credit(code, TransactionSubType::CheckPostedAndReturned),
            "257" => TransactionType::Credit(code, TransactionSubType::IndividualAchReturnItem),
            "258" => TransactionType::Credit(code, TransactionSubType::AchReversalCredit),
            "261" => TransactionType::Credit(code, TransactionSubType::IndividualRejectedCredit),
            "263" => TransactionType::Credit(code, TransactionSubType::Overdraft),
            "266" => TransactionType::Credit(code, TransactionSubType::ReturnItem),
            "268" => TransactionType::Credit(code, TransactionSubType::ReturnItemAdjustment),
            "274" => TransactionType::Credit(
                code,
                TransactionSubType::CumulativeZbaOrDisbursementCredits,
            ),
            "275" => TransactionType::Credit(code, TransactionSubType::ZbaCredit),
            "276" => TransactionType::Credit(code, TransactionSubType::ZbaFloatAdjustment),
            "277" => TransactionType::Credit(code, TransactionSubType::ZbaCreditTransfer),
            "278" => TransactionType::Credit(code, TransactionSubType::ZbaCreditAdjustment),
            "281" => TransactionType::Credit(
                code,
                TransactionSubType::IndividualControlledDisbursingCredit,
            ),
            "286" => {
                TransactionType::Credit(code, TransactionSubType::IndividualDtcDisbursingCredit)
            }
            "295" => TransactionType::Credit(code, TransactionSubType::AtmCredit),
            "301" => TransactionType::Credit(code, TransactionSubType::CommercialDeposit),
            "306" => TransactionType::Credit(code, TransactionSubType::FedFundsSold),
            "308" => TransactionType::Credit(code, TransactionSubType::TrustCredit),
            "331" => TransactionType::Credit(code, TransactionSubType::IndividualEscrowCredit),
            "342" => TransactionType::Credit(code, TransactionSubType::BrokerDeposit),
            "344" => TransactionType::Credit(code, TransactionSubType::IndividualBackValueCredit),
            "345" => TransactionType::Credit(code, TransactionSubType::ItemInBrokersDeposit),
            "346" => TransactionType::Credit(code, TransactionSubType::SweepInterestIncome),
            "347" => TransactionType::Credit(code, TransactionSubType::SweepPrincipalSell),
            "348" => TransactionType::Credit(code, TransactionSubType::FuturesCredit),
            "349" => TransactionType::Credit(code, TransactionSubType::PrincipalPaymentsCredit),
            "351" => TransactionType::Credit(code, TransactionSubType::IndividualInvestmentSold),
            "353" => TransactionType::Credit(code, TransactionSubType::CashCenterCredit),
            "354" => TransactionType::Credit(code, TransactionSubType::InterestCredit),
            "357" => TransactionType::Credit(code, TransactionSubType::CreditAdjustment),
            "358" => TransactionType::Credit(code, TransactionSubType::YtdAdjustmentCredit),
            "359" => TransactionType::Credit(code, TransactionSubType::InterestAdjustmentCredit),
            "362" => TransactionType::Credit(code, TransactionSubType::CorrespondentCollection),
            "363" => {
                TransactionType::Credit(code, TransactionSubType::CorrespondentCollectionAdjustment)
            }
            "364" => TransactionType::Credit(code, TransactionSubType::LoanParticipation),
            "366" => TransactionType::Credit(code, TransactionSubType::CurrencyAndCoinDeposited),
            "367" => TransactionType::Credit(code, TransactionSubType::FoodStampLetter),
            "368" => TransactionType::Credit(code, TransactionSubType::FoodStampAdjustment),
            "369" => TransactionType::Credit(code, TransactionSubType::ClearingSettlementCredit),
            "372" => TransactionType::Credit(code, TransactionSubType::BackValueAdjustment),
            "373" => TransactionType::Credit(code, TransactionSubType::CustomerPayroll),
            "374" => TransactionType::Credit(code, TransactionSubType::FrbStatementRecap),
            "376" => {
                TransactionType::Credit(code, TransactionSubType::SavingsBondLetterOrAdjustment)
            }
            "377" => TransactionType::Credit(code, TransactionSubType::TreasuryTaxAndLoanCredit),
            "378" => TransactionType::Credit(code, TransactionSubType::TransferOfTreasuryCredit),
            "379" => TransactionType::Credit(
                code,
                TransactionSubType::FrbGovernmentChecksCashLetterCredit,
            ),
            "381" => {
                TransactionType::Credit(code, TransactionSubType::FrbGovernmentCheckAdjustment)
            }
            "382" => TransactionType::Credit(code, TransactionSubType::FrbPostalMoneyOrderCredit),
            "383" => {
                TransactionType::Credit(code, TransactionSubType::FrbPostalMoneyOrderAdjustment)
            }
            "384" => {
                TransactionType::Credit(code, TransactionSubType::FrbCashLetterAutoChargeCredit)
            }
            "386" => {
                TransactionType::Credit(code, TransactionSubType::FrbCashLetterAutoChargeAdjustment)
            }
            "387" => TransactionType::Credit(code, TransactionSubType::FrbFineSortCashLetterCredit),
            "388" => TransactionType::Credit(code, TransactionSubType::FrbFineSortAdjustment),
            "391" => TransactionType::Credit(code, TransactionSubType::UniversalCredit),
            "392" => TransactionType::Credit(code, TransactionSubType::FreightPaymentCredit),
            "393" => TransactionType::Credit(code, TransactionSubType::ItemizedCreditOver10000),
            "394" => TransactionType::Credit(code, TransactionSubType::CumulativeCredits),
            "395" => TransactionType::Credit(code, TransactionSubType::CheckReversal),
            "397" => TransactionType::Credit(code, TransactionSubType::FloatAdjustment),
            "398" => TransactionType::Credit(code, TransactionSubType::MiscellaneousFeeRefund),
            "399" => TransactionType::Credit(code, TransactionSubType::MiscellaneousCredit),
            "408" => TransactionType::Debit(code, TransactionSubType::FloatAdjustment),
            "409" => TransactionType::Debit(code, TransactionSubType::DebitAnyType),
            "415" => TransactionType::Debit(code, TransactionSubType::LockboxDebit),
            "421" => TransactionType::Debit(code, TransactionSubType::EdiTransactionDebit),
            "422" => TransactionType::Debit(code, TransactionSubType::EdibanxSettlementDebit),
            "423" => TransactionType::Debit(code, TransactionSubType::EdibanxReturnItemDebit),
            "435" => TransactionType::Debit(code, TransactionSubType::PayableThroughDraft),
            "445" => TransactionType::Debit(code, TransactionSubType::AchConcentrationDebit),
            "447" => TransactionType::Debit(code, TransactionSubType::AchDisbursementFundingDebit),
            "451" => TransactionType::Debit(code, TransactionSubType::AchDebitReceived),
            "452" => TransactionType::Debit(code, TransactionSubType::ItemInAchDisbursementOrDebit),
            "455" => TransactionType::Debit(code, TransactionSubType::PreauthorizedAchDebit),
            "462" => {
                TransactionType::Debit(code, TransactionSubType::AccountHolderInitiatedAchDebit)
            }
            "464" => TransactionType::Debit(code, TransactionSubType::CorporateTradePaymentDebit),
            "466" => TransactionType::Debit(code, TransactionSubType::AchSettlement),
            "468" => TransactionType::Debit(
                code,
                TransactionSubType::AchReturnItemOrAdjustmentSettlement,
            ),
            "469" => TransactionType::Debit(code, TransactionSubType::MiscellaneousAchDebit),
            "472" => TransactionType::Debit(code, TransactionSubType::CumulativeChecksPaid),
            "474" => TransactionType::Debit(code, TransactionSubType::CertifiedCheckDebit),
            "475" => TransactionType::Debit(code, TransactionSubType::CheckPaid),
            "476" => {
                TransactionType::Debit(code, TransactionSubType::FederalReserveBankLetterDebit)
            }
            "477" => TransactionType::Debit(code, TransactionSubType::BankOriginatedDebit),
            "479" => TransactionType::Debit(code, TransactionSubType::ListPostDebit),
            "481" => TransactionType::Debit(code, TransactionSubType::IndividualLoanPayment),
            "484" => TransactionType::Debit(code, TransactionSubType::Draft),
            "485" => TransactionType::Debit(code, TransactionSubType::DtcDebit),
            "487" => TransactionType::Debit(code, TransactionSubType::CashLetterDebit),
            "489" => TransactionType::Debit(code, TransactionSubType::CashLetterAdjustment),
            "491" => TransactionType::Debit(
                code,
                TransactionSubType::IndividualOutgoingInternalMoneyTransfer,
            ),
            "493" => TransactionType::Debit(
                code,
                TransactionSubType::CustomerTerminalInitiatedMoneyTransfer,
            ),
            "495" => TransactionType::Debit(code, TransactionSubType::OutgoingMoneyTransfer),
            "496" => TransactionType::Debit(code, TransactionSubType::MoneyTransferAdjustment),
            "498" => TransactionType::Debit(code, TransactionSubType::Compensation),
            "501" => {
                TransactionType::Debit(code, TransactionSubType::IndividualAutomaticTransferDebit)
            }
            "502" => TransactionType::Debit(code, TransactionSubType::BondOperationsDebit),
            "506" => TransactionType::Debit(code, TransactionSubType::BookTransferDebit),
            "508" => TransactionType::Debit(
                code,
                TransactionSubType::IndividualInternationalMoneyTransferDebits,
            ),
            "512" => TransactionType::Debit(code, TransactionSubType::LetterOfCreditDebit),
            "513" => TransactionType::Debit(code, TransactionSubType::LetterOfCredit),
            "514" => TransactionType::Debit(code, TransactionSubType::ForeignExchangeDebit),
            "516" => TransactionType::Debit(code, TransactionSubType::ForeignRemittanceDebit),
            "518" => TransactionType::Debit(code, TransactionSubType::ForeignCollectionDebit),
            "522" => TransactionType::Debit(code, TransactionSubType::ForeignChecksPaid),
            "524" => TransactionType::Debit(code, TransactionSubType::Commission),
            "526" => {
                TransactionType::Debit(code, TransactionSubType::InternationalMoneyMarketTrading)
            }
            "527" => TransactionType::Debit(code, TransactionSubType::StandingOrder),
            "529" => {
                TransactionType::Debit(code, TransactionSubType::MiscellaneousInternationalDebit)
            }
            "531" => TransactionType::Debit(code, TransactionSubType::SecuritiesPurchased),
            "533" => TransactionType::Debit(code, TransactionSubType::SecurityCollectionDebit),
            "535" => TransactionType::Debit(code, TransactionSubType::PurchaseOfEquitySecurities),
            "538" => TransactionType::Debit(code, TransactionSubType::MaturedRepurchaseOrder),
            "540" => TransactionType::Debit(code, TransactionSubType::CouponCollectionDebit),
            "541" => TransactionType::Debit(code, TransactionSubType::BankersAcceptances),
            "542" => TransactionType::Debit(code, TransactionSubType::PurchaseOfDebtSecurities),
            "543" => TransactionType::Debit(code, TransactionSubType::DomesticCollection),
            "544" => {
                TransactionType::Debit(code, TransactionSubType::InterestMaturedPrincipalPayment)
            }
            "546" => TransactionType::Debit(code, TransactionSubType::CommercialPaper),
            "547" => TransactionType::Debit(code, TransactionSubType::CapitalChange),
            "548" => TransactionType::Debit(code, TransactionSubType::SavingsBondsSalesAdjustment),
            "549" => TransactionType::Debit(code, TransactionSubType::MiscellaneousSecurityDebit),
            "552" => TransactionType::Debit(code, TransactionSubType::CreditReversal),
            "554" => TransactionType::Debit(code, TransactionSubType::PostingErrorCorrectionDebit),
            "555" => TransactionType::Debit(code, TransactionSubType::DepositedItemReturned),
            "557" => TransactionType::Debit(code, TransactionSubType::IndividualAchReturnItem),
            "558" => TransactionType::Debit(code, TransactionSubType::AchReversalDebit),
            "561" => TransactionType::Debit(code, TransactionSubType::IndividualRejectedDebit),
            "563" => TransactionType::Debit(code, TransactionSubType::Overdraft),
            "564" => TransactionType::Debit(code, TransactionSubType::OverdraftFee),
            "566" => TransactionType::Debit(code, TransactionSubType::ReturnItem),
            "567" => TransactionType::Debit(code, TransactionSubType::ReturnItemFee),
            "568" => TransactionType::Debit(code, TransactionSubType::ReturnItemAdjustment),
            "574" => TransactionType::Debit(code, TransactionSubType::CumulativeZbaDebits),
            "575" => TransactionType::Debit(code, TransactionSubType::ZbaDebit),
            "577" => TransactionType::Debit(code, TransactionSubType::ZbaDebitTransfer),
            "578" => TransactionType::Debit(code, TransactionSubType::ZbaDebitAdjustment),
            "581" => TransactionType::Debit(
                code,
                TransactionSubType::IndividualControlledDisbursingDebit,
            ),
            "595" => TransactionType::Debit(code, TransactionSubType::AtmDebit),
            "597" => TransactionType::Debit(code, TransactionSubType::ArpDebit),
            "616" => TransactionType::Debit(
                code,
                TransactionSubType::FederalReserveBankCommercialBankDebit,
            ),
            "622" => TransactionType::Debit(code, TransactionSubType::BrokerDebit),
            "627" => TransactionType::Debit(code, TransactionSubType::FedFundsPurchased),
            "629" => TransactionType::Debit(code, TransactionSubType::CashCenterDebit),
            "631" => TransactionType::Debit(code, TransactionSubType::DebitAdjustment),
            "633" => TransactionType::Debit(code, TransactionSubType::TrustDebit),
            "634" => TransactionType::Debit(code, TransactionSubType::YtdAdjustmentDebit),
            "641" => TransactionType::Debit(code, TransactionSubType::IndividualEscrowDebit),
            "644" => TransactionType::Debit(code, TransactionSubType::IndividualBackValueDebit),
            "651" => {
                TransactionType::Debit(code, TransactionSubType::IndividualInvestmentPurchased)
            }
            "654" => TransactionType::Debit(code, TransactionSubType::InterestDebit),
            "656" => TransactionType::Debit(code, TransactionSubType::SweepPrincipalBuy),
            "657" => TransactionType::Debit(code, TransactionSubType::FuturesDebit),
            "658" => TransactionType::Debit(code, TransactionSubType::PrincipalPaymentsDebit),
            "659" => TransactionType::Debit(code, TransactionSubType::InterestAdjustmentDebit),
            "661" => TransactionType::Debit(code, TransactionSubType::AccountAnalysisFee),
            "662" => TransactionType::Debit(code, TransactionSubType::CorrespondentCollectionDebit),
            "663" => {
                TransactionType::Debit(code, TransactionSubType::CorrespondentCollectionAdjustment)
            }
            "664" => TransactionType::Debit(code, TransactionSubType::LoanParticipation),
            "666" => TransactionType::Debit(code, TransactionSubType::CurrencyAndCoinShipped),
            "667" => TransactionType::Debit(code, TransactionSubType::FoodStampLetter),
            "668" => TransactionType::Debit(code, TransactionSubType::FoodStampAdjustment),
            "669" => TransactionType::Debit(code, TransactionSubType::ClearingSettlementDebit),
            "672" => TransactionType::Debit(code, TransactionSubType::BackValueAdjustment),
            "673" => TransactionType::Debit(code, TransactionSubType::CustomerPayroll),
            "674" => TransactionType::Debit(code, TransactionSubType::FrbStatementRecap),
            "676" => {
                TransactionType::Debit(code, TransactionSubType::SavingsBondLetterOrAdjustment)
            }
            "677" => TransactionType::Debit(code, TransactionSubType::TreasuryTaxAndLoanDebit),
            "678" => TransactionType::Debit(code, TransactionSubType::TransferOfTreasuryDebit),
            "679" => {
                TransactionType::Debit(code, TransactionSubType::FrbGovernmentChecksCashLetterDebit)
            }
            "681" => TransactionType::Debit(code, TransactionSubType::FrbGovernmentCheckAdjustment),
            "682" => TransactionType::Debit(code, TransactionSubType::FrbPostalMoneyOrderDebit),
            "683" => {
                TransactionType::Debit(code, TransactionSubType::FrbPostalMoneyOrderAdjustment)
            }
            "684" => TransactionType::Debit(code, TransactionSubType::FrbCashLetterAutoChargeDebit),
            "686" => {
                TransactionType::Debit(code, TransactionSubType::FrbCashLetterAutoChargeAdjustment)
            }
            "687" => TransactionType::Debit(code, TransactionSubType::FrbFineSortCashLetterDebit),
            "688" => TransactionType::Debit(code, TransactionSubType::FrbFineSortAdjustment),
            "691" => TransactionType::Debit(code, TransactionSubType::UniversalDebit),
            "692" => TransactionType::Debit(code, TransactionSubType::FreightPaymentDebit),
            "693" => TransactionType::Debit(code, TransactionSubType::ItemizedDebitOver10000),
            "694" => TransactionType::Debit(code, TransactionSubType::DepositReversal),
            "695" => TransactionType::Debit(code, TransactionSubType::DepositCorrectionDebit),
            "696" => TransactionType::Debit(code, TransactionSubType::RegularCollectionDebit),
            "697" => TransactionType::Debit(code, TransactionSubType::CumulativeDebits),
            "698" => TransactionType::Debit(code, TransactionSubType::MiscellaneousFees),
            "699" => TransactionType::Debit(code, TransactionSubType::MiscellaneousDebit),
            "721" => TransactionType::Credit(code, TransactionSubType::AmountAppliedToInterest),
            "722" => TransactionType::Credit(code, TransactionSubType::AmountAppliedToPrincipal),
            "723" => TransactionType::Credit(code, TransactionSubType::AmountAppliedToEscrow),
            "724" => TransactionType::Credit(code, TransactionSubType::AmountAppliedToLateCharges),
            "725" => TransactionType::Credit(code, TransactionSubType::AmountAppliedToBuydown),
            "726" => TransactionType::Credit(code, TransactionSubType::AmountAppliedToMiscFees),
            "727" => TransactionType::Credit(
                code,
                TransactionSubType::AmountAppliedToDeferredInterestDetail,
            ),
            "728" => {
                TransactionType::Credit(code, TransactionSubType::AmountAppliedToServiceCharge)
            }
            "890" => TransactionType::Unknown(code, TransactionSubType::Info),
            other_code => match other_code.parse::<i16>() {
                Ok(n) if n >= 920 && n <= 959 => {
                    return TransactionType::Credit(code, TransactionSubType::Custom);
                }
                Ok(n) if n >= 960 && n <= 999 => {
                    return TransactionType::Debit(code, TransactionSubType::Custom);
                }
                _ => TransactionType::Unknown(code, TransactionSubType::Unknown),
            },
        }
    }
}

impl Serialize for TransactionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("TransactionType", 3)?;

        match *self {
            TransactionType::Credit(ref c, ref t) => {
                state.serialize_field("code", c)?;
                state.serialize_field("direction", "credit")?;
                state.serialize_field("type", t)?;
            }
            TransactionType::Debit(ref c, ref t) => {
                state.serialize_field("code", c)?;
                state.serialize_field("direction", "debit")?;
                state.serialize_field("type", t)?;
            }
            TransactionType::Unknown(ref c, ref t) => {
                state.serialize_field("code", c)?;
                state.serialize_field("direction", "unknown")?;
                state.serialize_field("type", t)?;
            }
        }

        state.end()
    }
}
