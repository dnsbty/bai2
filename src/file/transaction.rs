use chrono::NaiveDate;
use serde::Serialize;
use std::collections::HashMap;

use super::funds_type::{FundsSubType, FundsType};
use super::util::{parse_date, parse_int, parse_string, parse_time};

use crate::scanner::node::Node;

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
enum Direction {
    Credit,
    Debit,
    Unknown,
}

#[derive(Debug, Serialize)]
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
    fn parse(type_code: &str) -> (Direction, TransactionType) {
        match type_code {
            "108" => (Direction::Credit, TransactionType::Credit),
            "115" => (Direction::Credit, TransactionType::LockboxDeposit),
            "116" => (Direction::Credit, TransactionType::ItemInLockboxDeposit),
            "118" => (Direction::Credit, TransactionType::LockboxAdjustmentCredit),
            "121" => (Direction::Credit, TransactionType::EdiTransactionCredit),
            "122" => (Direction::Credit, TransactionType::EdibanxCreditReceived),
            "123" => (Direction::Credit, TransactionType::EdibanxCreditReturn),
            "135" => (Direction::Credit, TransactionType::DtcConcentrationCredit),
            "136" => (Direction::Credit, TransactionType::ItemInDtcDeposit),
            "142" => (Direction::Credit, TransactionType::AchCreditReceived),
            "143" => (Direction::Credit, TransactionType::ItemInAchDeposit),
            "145" => (Direction::Credit, TransactionType::AchConcentrationCredit),
            "147" => (
                Direction::Credit,
                TransactionType::IndividualBankCardDeposit,
            ),
            "155" => (Direction::Credit, TransactionType::PreauthorizedDraftCredit),
            "156" => (Direction::Credit, TransactionType::ItemInPacDeposit),
            "164" => (
                Direction::Credit,
                TransactionType::CorporateTradePaymentCredit,
            ),
            "165" => (Direction::Credit, TransactionType::PreauthorizedAchCredit),
            "166" => (Direction::Credit, TransactionType::AchSettlement),
            "168" => (
                Direction::Credit,
                TransactionType::AchReturnItemOrAdjustmentSettlement,
            ),
            "169" => (Direction::Credit, TransactionType::MiscellaneousAchCredit),
            "171" => (Direction::Credit, TransactionType::IndividualLoanDeposit),
            "172" => (Direction::Credit, TransactionType::DepositCorrection),
            "173" => (Direction::Credit, TransactionType::BankPreparedDeposit),
            "174" => (Direction::Credit, TransactionType::OtherDeposit),
            "175" => (Direction::Credit, TransactionType::CheckDepositPackage),
            "176" => (Direction::Credit, TransactionType::RePresentedCheckDeposit),
            "184" => (Direction::Credit, TransactionType::DraftDeposit),
            "187" => (Direction::Credit, TransactionType::CashLetterCredit),
            "189" => (Direction::Credit, TransactionType::CashLetterAdjustment),
            "191" => (
                Direction::Credit,
                TransactionType::IndividualIncomingInternalMoneyTransfer,
            ),
            "195" => (Direction::Credit, TransactionType::IncomingMoneyTransfer),
            "196" => (Direction::Credit, TransactionType::MoneyTransferAdjustment),
            "198" => (Direction::Credit, TransactionType::Compensation),
            "201" => (
                Direction::Credit,
                TransactionType::IndividualAutomaticTransferCredit,
            ),
            "202" => (Direction::Credit, TransactionType::BondOperationsCredit),
            "206" => (Direction::Credit, TransactionType::BookTransferCredit),
            "208" => (
                Direction::Credit,
                TransactionType::IndividualInternationalMoneyTransferCredit,
            ),
            "212" => (Direction::Credit, TransactionType::ForeignLetterOfCredit),
            "213" => (Direction::Credit, TransactionType::LetterOfCredit),
            "214" => (Direction::Credit, TransactionType::ForeignExchangeOfCredit),
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
            "252" => (Direction::Credit, TransactionType::DebitReversal),
            "254" => (
                Direction::Credit,
                TransactionType::PostingErrorCorrectionCredit,
            ),
            "255" => (Direction::Credit, TransactionType::CheckPostedAndReturned),
            "257" => (Direction::Credit, TransactionType::IndividualAchReturnItem),
            "258" => (Direction::Credit, TransactionType::AchReversalCredit),
            "261" => (Direction::Credit, TransactionType::IndividualRejectedCredit),
            "263" => (Direction::Credit, TransactionType::Overdraft),
            "266" => (Direction::Credit, TransactionType::ReturnItem),
            "268" => (Direction::Credit, TransactionType::ReturnItemAdjustment),
            "274" => (
                Direction::Credit,
                TransactionType::CumulativeZbaOrDisbursementCredits,
            ),
            "275" => (Direction::Credit, TransactionType::ZbaCredit),
            "276" => (Direction::Credit, TransactionType::ZbaFloatAdjustment),
            "277" => (Direction::Credit, TransactionType::ZbaCreditTransfer),
            "278" => (Direction::Credit, TransactionType::ZbaCreditAdjustment),
            "281" => (
                Direction::Credit,
                TransactionType::IndividualControlledDisbursingCredit,
            ),
            "286" => (
                Direction::Credit,
                TransactionType::IndividualDtcDisbursingCredit,
            ),
            "295" => (Direction::Credit, TransactionType::AtmCredit),
            "301" => (Direction::Credit, TransactionType::CommercialDeposit),
            "306" => (Direction::Credit, TransactionType::FedFundsSold),
            "308" => (Direction::Credit, TransactionType::TrustCredit),
            "331" => (Direction::Credit, TransactionType::IndividualEscrowCredit),
            "342" => (Direction::Credit, TransactionType::BrokerDeposit),
            "344" => (
                Direction::Credit,
                TransactionType::IndividualBackValueCredit,
            ),
            "345" => (Direction::Credit, TransactionType::ItemInBrokersDeposit),
            "346" => (Direction::Credit, TransactionType::SweepInterestIncome),
            "347" => (Direction::Credit, TransactionType::SweepPrincipalSell),
            "348" => (Direction::Credit, TransactionType::FuturesCredit),
            "349" => (Direction::Credit, TransactionType::PrincipalPaymentsCredit),
            "351" => (Direction::Credit, TransactionType::IndividualInvestmentSold),
            "353" => (Direction::Credit, TransactionType::CashCenterCredit),
            "354" => (Direction::Credit, TransactionType::InterestCredit),
            "357" => (Direction::Credit, TransactionType::CreditAdjustment),
            "358" => (Direction::Credit, TransactionType::YtdAdjustmentCredit),
            "359" => (Direction::Credit, TransactionType::InterestAdjustmentCredit),
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
            "386" => (
                Direction::Credit,
                TransactionType::FrbCashLetterAutoChargeAdjustment,
            ),
            "387" => (
                Direction::Credit,
                TransactionType::FrbFineSortCashLetterCredit,
            ),
            "388" => (Direction::Credit, TransactionType::FrbFineSortAdjustment),
            "391" => (Direction::Credit, TransactionType::UniversalCredit),
            "392" => (Direction::Credit, TransactionType::FreightPaymentCredit),
            "393" => (Direction::Credit, TransactionType::ItemizedCreditOver10000),
            "394" => (Direction::Credit, TransactionType::CumulativeCredits),
            "395" => (Direction::Credit, TransactionType::CheckReversal),
            "397" => (Direction::Credit, TransactionType::FloatAdjustment),
            "398" => (Direction::Credit, TransactionType::MiscellaneousFeeRefund),
            "399" => (Direction::Credit, TransactionType::MiscellaneousCredit),
            "408" => (Direction::Debit, TransactionType::FloatAdjustment),
            "409" => (Direction::Debit, TransactionType::DebitAnyType),
            "415" => (Direction::Debit, TransactionType::LockboxDebit),
            "421" => (Direction::Debit, TransactionType::EdiTransactionDebit),
            "422" => (Direction::Debit, TransactionType::EdibanxSettlementDebit),
            "423" => (Direction::Debit, TransactionType::EdibanxReturnItemDebit),
            "435" => (Direction::Debit, TransactionType::PayableThroughDraft),
            "445" => (Direction::Debit, TransactionType::AchConcentrationDebit),
            "447" => (
                Direction::Debit,
                TransactionType::AchDisbursementFundingDebit,
            ),
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
            "464" => (
                Direction::Debit,
                TransactionType::CorporateTradePaymentDebit,
            ),
            "466" => (Direction::Debit, TransactionType::AchSettlement),
            "468" => (
                Direction::Debit,
                TransactionType::AchReturnItemOrAdjustmentSettlement,
            ),
            "469" => (Direction::Debit, TransactionType::MiscellaneousAchDebit),
            "472" => (Direction::Debit, TransactionType::CumulativeChecksPaid),
            "474" => (Direction::Debit, TransactionType::CertifiedCheckDebit),
            "475" => (Direction::Debit, TransactionType::CheckPaid),
            "476" => (
                Direction::Debit,
                TransactionType::FederalReserveBankLetterDebit,
            ),
            "477" => (Direction::Debit, TransactionType::BankOriginatedDebit),
            "479" => (Direction::Debit, TransactionType::ListPostDebit),
            "481" => (Direction::Debit, TransactionType::IndividualLoanPayment),
            "484" => (Direction::Debit, TransactionType::Draft),
            "485" => (Direction::Debit, TransactionType::DtcDebit),
            "487" => (Direction::Debit, TransactionType::CashLetterDebit),
            "489" => (Direction::Debit, TransactionType::CashLetterAdjustment),
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
            "501" => (
                Direction::Debit,
                TransactionType::IndividualAutomaticTransferDebit,
            ),
            "502" => (Direction::Debit, TransactionType::BondOperationsDebit),
            "506" => (Direction::Debit, TransactionType::BookTransferDebit),
            "508" => (
                Direction::Debit,
                TransactionType::IndividualInternationalMoneyTransferDebits,
            ),
            "512" => (Direction::Debit, TransactionType::LetterOfCreditDebit),
            "513" => (Direction::Debit, TransactionType::LetterOfCredit),
            "514" => (Direction::Debit, TransactionType::ForeignExchangeDebit),
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
            "531" => (Direction::Debit, TransactionType::SecuritiesPurchased),
            "533" => (Direction::Debit, TransactionType::SecurityCollectionDebit),
            "535" => (
                Direction::Debit,
                TransactionType::PurchaseOfEquitySecurities,
            ),
            "538" => (Direction::Debit, TransactionType::MaturedRepurchaseOrder),
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
            "552" => (Direction::Debit, TransactionType::CreditReversal),
            "554" => (
                Direction::Debit,
                TransactionType::PostingErrorCorrectionDebit,
            ),
            "555" => (Direction::Debit, TransactionType::DepositedItemReturned),
            "557" => (Direction::Debit, TransactionType::IndividualAchReturnItem),
            "558" => (Direction::Debit, TransactionType::AchReversalDebit),
            "561" => (Direction::Debit, TransactionType::IndividualRejectedDebit),
            "563" => (Direction::Debit, TransactionType::Overdraft),
            "564" => (Direction::Debit, TransactionType::OverdraftFee),
            "566" => (Direction::Debit, TransactionType::ReturnItem),
            "567" => (Direction::Debit, TransactionType::ReturnItemFee),
            "568" => (Direction::Debit, TransactionType::ReturnItemAdjustment),
            "574" => (Direction::Debit, TransactionType::CumulativeZbaDebits),
            "575" => (Direction::Debit, TransactionType::ZbaDebit),
            "577" => (Direction::Debit, TransactionType::ZbaDebitTransfer),
            "578" => (Direction::Debit, TransactionType::ZbaDebitAdjustment),
            "581" => (
                Direction::Debit,
                TransactionType::IndividualControlledDisbursingDebit,
            ),
            "595" => (Direction::Debit, TransactionType::AtmDebit),
            "597" => (Direction::Debit, TransactionType::ArpDebit),
            "616" => (
                Direction::Debit,
                TransactionType::FederalReserveBankCommercialBankDebit,
            ),
            "622" => (Direction::Debit, TransactionType::BrokerDebit),
            "627" => (Direction::Debit, TransactionType::FedFundsPurchased),
            "629" => (Direction::Debit, TransactionType::CashCenterDebit),
            "631" => (Direction::Debit, TransactionType::DebitAdjustment),
            "633" => (Direction::Debit, TransactionType::TrustDebit),
            "634" => (Direction::Debit, TransactionType::YtdAdjustmentDebit),
            "641" => (Direction::Debit, TransactionType::IndividualEscrowDebit),
            "644" => (Direction::Debit, TransactionType::IndividualBackValueDebit),
            "651" => (
                Direction::Debit,
                TransactionType::IndividualInvestmentPurchased,
            ),
            "654" => (Direction::Debit, TransactionType::InterestDebit),
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
            "666" => (Direction::Debit, TransactionType::CurrencyAndCoinShipped),
            "667" => (Direction::Debit, TransactionType::FoodStampLetter),
            "668" => (Direction::Debit, TransactionType::FoodStampAdjustment),
            "669" => (Direction::Debit, TransactionType::ClearingSettlementDebit),
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
            "686" => (
                Direction::Debit,
                TransactionType::FrbCashLetterAutoChargeAdjustment,
            ),
            "687" => (
                Direction::Debit,
                TransactionType::FrbFineSortCashLetterDebit,
            ),
            "688" => (Direction::Debit, TransactionType::FrbFineSortAdjustment),
            "691" => (Direction::Debit, TransactionType::UniversalDebit),
            "692" => (Direction::Debit, TransactionType::FreightPaymentDebit),
            "693" => (Direction::Debit, TransactionType::ItemizedDebitOver10000),
            "694" => (Direction::Debit, TransactionType::DepositReversal),
            "695" => (Direction::Debit, TransactionType::DepositCorrectionDebit),
            "696" => (Direction::Debit, TransactionType::RegularCollectionDebit),
            "697" => (Direction::Debit, TransactionType::CumulativeDebits),
            "698" => (Direction::Debit, TransactionType::MiscellaneousFees),
            "699" => (Direction::Debit, TransactionType::MiscellaneousDebit),
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
            "890" => (Direction::Unknown, TransactionType::Info),
            code => match code.parse::<i16>() {
                Ok(n) if n >= 920 && n <= 959 => {
                    return (Direction::Credit, TransactionType::Custom);
                }
                Ok(n) if n >= 960 && n <= 999 => {
                    return (Direction::Debit, TransactionType::Custom);
                }
                _ => (Direction::Unknown, TransactionType::Unknown),
            },
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Transaction {
    amount: Option<u64>,
    availability: HashMap<u16, i64>,
    bank_reference_number: String,
    customer_reference_number: String,
    direction: Direction,
    funds_type: FundsType,
    text: Vec<String>,
    transaction_type: TransactionType,
    transaction_type_code: String,
    value_date: Option<NaiveDate>,
    value_time: Option<String>,
}

impl Transaction {
    pub fn from_node(node: &Node) -> Result<Transaction, &'static str> {
        let fields = node.fields();
        let num_fields = fields.len();

        let transaction_type_code = parse_string(fields[1]);
        let (direction, transaction_type) = TransactionType::parse(&transaction_type_code);

        let mut next_start_index = 4;
        let mut value_date: Option<NaiveDate> = None;
        let mut value_time: Option<String> = None;
        let mut availability: HashMap<u16, i64> = HashMap::new();
        let funds_type = FundsType::parse(fields.get(3).unwrap_or(&""));

        match funds_type {
            FundsType::ValueDated => {
                value_date = parse_date(fields[next_start_index]);
                value_time = parse_time(fields[next_start_index + 1]);
                next_start_index = next_start_index + 2;
            }
            FundsType::DistributedAvailability(FundsSubType::S) => {
                availability.insert(0, parse_int(fields[next_start_index]).unwrap());
                availability.insert(1, parse_int(fields[next_start_index + 1]).unwrap());
                availability.insert(2, parse_int(fields[next_start_index + 2]).unwrap());
                next_start_index = next_start_index + 3;
            }
            FundsType::DistributedAvailability(FundsSubType::D) => {
                let num_distributions = parse_int(fields[next_start_index]).unwrap_or(0);
                next_start_index = next_start_index + 1;

                for _ in 0..num_distributions {
                    match (
                        parse_int(fields[next_start_index]),
                        parse_int(fields[next_start_index + 1]),
                    ) {
                        (Some(days), Some(amt)) => {
                            availability.insert(days, amt);
                        }
                        _ => {}
                    }

                    next_start_index = next_start_index + 2;
                }
            }
            _ => (),
        }

        let raw_bank_ref = fields.get(next_start_index).unwrap_or(&"");
        let raw_customer_ref = fields.get(next_start_index + 1).unwrap_or(&"");
        next_start_index += 2;

        let num_remaining_fields = num_fields - next_start_index;
        let mut text = Vec::new();

        for i in 0..num_remaining_fields {
            let raw_text = fields.get(next_start_index + i).unwrap_or(&"");
            let parsed_text = parse_string(raw_text);
            text.push(parsed_text);
        }

        Ok(Transaction {
            amount: parse_int(fields[2]),
            availability: HashMap::new(),
            bank_reference_number: parse_string(raw_bank_ref),
            customer_reference_number: parse_string(raw_customer_ref),
            direction,
            funds_type,
            text,
            transaction_type,
            transaction_type_code,
            value_date,
            value_time,
        })
    }
}
