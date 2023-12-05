use chrono::NaiveDate;
use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;
use std::collections::HashMap;

use crate::scanner::node::Node;

use super::funds_type::{FundsSubType, FundsType};
use super::transaction::Transaction;
use super::util::{parse_currency, parse_date, parse_int, parse_string, parse_time};

#[derive(Debug, Serialize)]
pub struct Account {
    amounts: Vec<Amount>,
    currency_code: String,
    customer_account_number: String,
    number_of_records: Option<u32>,
    transactions: Vec<Transaction>,
    total: Option<u64>,
    value_date: Option<NaiveDate>,
    value_time: Option<String>,
}

impl Account {
    pub fn from_node(node: &Node, default_currency: &str) -> Result<Account, &'static str> {
        let header_fields = node.fields();
        if header_fields.len() < 7 {
            return Err("Invalid account header. Expected 7 fields, but found less.");
        }

        let trailer_fields = node.sibling_fields();
        if trailer_fields.len() < 3 {
            return Err("Invalid account trailer. Expected 3 fields, but found less.");
        }

        let txns_result = node
            .children
            .iter()
            .map(Transaction::from_node)
            .collect::<Result<Vec<Transaction>, &'static str>>();

        match txns_result {
            Err(e) => Err(e),
            Ok(transactions) => Ok(Account {
                amounts: Amount::parse(header_fields[3..].to_vec()),
                currency_code: parse_currency(header_fields[2], default_currency),
                customer_account_number: parse_string(header_fields[1]),
                number_of_records: parse_int(trailer_fields[2]),
                transactions,
                total: parse_int(trailer_fields[1]),
                value_date: None,
                value_time: None,
            }),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Amount {
    amount_type: AmountType,
    amount_type_code: String,
    amount: Option<i64>,
    availability: HashMap<u16, i64>,
    funds_type: FundsType,
    item_count: Option<u16>,
    value_date: Option<NaiveDate>,
    value_time: Option<String>,
}

impl Amount {
    fn parse(fields: Vec<&str>) -> Vec<Amount> {
        let mut amounts = Vec::new();
        let mut next_start_index = 0;

        while fields.len() > next_start_index + 1 {
            let amount_type_code = parse_string(fields[next_start_index]);
            let mut amount = Amount {
                amount: parse_int(fields[next_start_index + 1]),
                amount_type: AmountType::parse(&amount_type_code),
                amount_type_code,
                availability: HashMap::new(),
                funds_type: FundsType::parse(fields[next_start_index + 3]),
                item_count: parse_int(fields[next_start_index + 2]),
                value_date: None,
                value_time: None,
            };

            match amount.funds_type {
                FundsType::ValueDated => {
                    amount.value_date = parse_date(fields[next_start_index + 4]);
                    amount.value_time = parse_time(fields[next_start_index + 5]);
                    next_start_index = next_start_index + 6;
                }
                FundsType::DistributedAvailability(FundsSubType::S) => {
                    amount
                        .availability
                        .insert(0, parse_int(fields[next_start_index + 4]).unwrap());
                    amount
                        .availability
                        .insert(1, parse_int(fields[next_start_index + 5]).unwrap());
                    amount
                        .availability
                        .insert(2, parse_int(fields[next_start_index + 6]).unwrap());
                    next_start_index = next_start_index + 7;
                }
                FundsType::DistributedAvailability(FundsSubType::D) => {
                    let num_distributions = parse_int(fields[next_start_index + 4]).unwrap_or(0);
                    next_start_index = next_start_index + 5;

                    for _ in 0..num_distributions {
                        match (
                            parse_int(fields[next_start_index]),
                            parse_int(fields[next_start_index + 1]),
                        ) {
                            (Some(days), Some(amt)) => {
                                amount.availability.insert(days, amt);
                            }
                            _ => {}
                        }

                        next_start_index = next_start_index + 2;
                    }
                }
                _ => {
                    next_start_index = next_start_index + 4;
                }
            }

            amounts.push(amount);
        }

        return amounts;
    }
}

#[derive(Debug)]
pub enum AmountType {
    Status(String, AmountSubtype),
    CreditSummary(String, AmountSubtype),
    DebitSummary(String, AmountSubtype),
    Unknown(String, AmountSubtype),
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AmountSubtype {
    AchNetPosition,
    AchSettlementCredits,
    AchSettlementDebits,
    AdjustedBalance,
    AdjustedBalanceMtd,
    AdjustedBalanceYtd,
    AdjustedTotalDisbursement,
    AdjustmentToBalances,
    AggregateBalanceAdjustments,
    AvailableCommitmentAmount,
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
    CorporateTradePaymentCredits,
    CorporateTradePaymentDebits,
    CorporateTradePaymentSettlement,
    CorrespondentBankDeposit,
    CreditsNotDetailed,
    CurrentAvailable,
    CurrentAvailableCrsSupressed,
    CurrentDayTotalLockboxDeposits,
    CurrentLedger,
    CustomCreditSummary,
    CustomDebitSummary,
    CustomStatus,
    DebitsNotDetailed,
    DepositsSubjectToFloat,
    DisbursingFundingRequirement,
    DisbursingOpeningAvailableBalance,
    EdiTransactionCredit,
    EdiTransactionDebits,
    EstimatedTotalDisbursement,
    FiveDayFloat,
    FloatAdjustment,
    FourDayFloat,
    FrbFreightPaymentDebits,
    FrbPresentmentEstimate,
    GrandTotalCreditsLessGrandTotalDebits,
    InterceptDebits,
    InterestAmountPastDue,
    InvestmentInterest,
    InvestmentSold,
    InvestmentsPurchased,
    LateDebitsAfterNotification,
    LateDeposit,
    ListPostCredits,
    ListPostDebits,
    LoanBalance,
    LoanDisbursement,
    MonthlyDividends,
    NetZeroBalanceAmount,
    OneDayFloat,
    OpeningAvailable,
    OpeningAvailableAndTotalSameDayAchDtcDeposit,
    OpeningLedger,
    PaymentAmountDue,
    PrincipalAmountPastDue,
    PrincipalLoanBalance,
    SixDayFloat,
    TargetBalance,
    ThreeOrMoreDaysFloat,
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
    TotalDebits,
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
    TotalFloat,
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
    TotalInvestmentPosition,
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
    TransferCalculation,
    TransferCalculationDebit,
    TwoOrMoreDaysFloat,
    Unknown,
    ZeroDayFloat,
}

impl AmountType {
    fn parse(type_code: &str) -> AmountType {
        let code = type_code.to_string();

        match type_code {
            "010" => AmountType::Status(code, AmountSubtype::OpeningLedger),
            "011" => AmountType::Status(code, AmountSubtype::AverageOpeningLedgerMtd),
            "012" => AmountType::Status(code, AmountSubtype::AverageOpeningLedgerYtd),
            "015" => AmountType::Status(code, AmountSubtype::ClosingLedger),
            "020" => AmountType::Status(code, AmountSubtype::AverageClosingLedgerMtd),
            "021" => AmountType::Status(code, AmountSubtype::AverageClosingLedgerPreviousMonth),
            "022" => AmountType::Status(code, AmountSubtype::AggregateBalanceAdjustments),
            "024" => AmountType::Status(code, AmountSubtype::AverageClosingLedgerYtdPreviousMonth),
            "025" => AmountType::Status(code, AmountSubtype::AverageClosingLedgerYtd),
            "030" => AmountType::Status(code, AmountSubtype::CurrentLedger),
            "037" => AmountType::Status(code, AmountSubtype::AchNetPosition),
            "039" => AmountType::Status(
                code,
                AmountSubtype::OpeningAvailableAndTotalSameDayAchDtcDeposit,
            ),
            "040" => AmountType::Status(code, AmountSubtype::OpeningAvailable),
            "041" => AmountType::Status(code, AmountSubtype::AverageOpeningAvailableMtd),
            "042" => AmountType::Status(code, AmountSubtype::AverageOpeningAvailableYtd),
            "043" => AmountType::Status(code, AmountSubtype::AverageAvailablePreviousMonth),
            "044" => AmountType::Status(code, AmountSubtype::DisbursingOpeningAvailableBalance),
            "045" => AmountType::Status(code, AmountSubtype::ClosingAvailable),
            "050" => AmountType::Status(code, AmountSubtype::AverageClosingAvailableMtd),
            "051" => AmountType::Status(code, AmountSubtype::AverageClosingAvailableLastMonth),
            "054" => AmountType::Status(code, AmountSubtype::AverageClosingAvailableYtdLastMonth),
            "055" => AmountType::Status(code, AmountSubtype::AverageClosingAvailableYtd),
            "056" => AmountType::Status(code, AmountSubtype::LoanBalance),
            "057" => AmountType::Status(code, AmountSubtype::TotalInvestmentPosition),
            "059" => AmountType::Status(code, AmountSubtype::CurrentAvailableCrsSupressed),
            "060" => AmountType::Status(code, AmountSubtype::CurrentAvailable),
            "061" => AmountType::Status(code, AmountSubtype::AverageCurrentAvailableMtd),
            "062" => AmountType::Status(code, AmountSubtype::AverageCurrentAvailableYtd),
            "063" => AmountType::Status(code, AmountSubtype::TotalFloat),
            "065" => AmountType::Status(code, AmountSubtype::TargetBalance),
            "066" => AmountType::Status(code, AmountSubtype::AdjustedBalance),
            "067" => AmountType::Status(code, AmountSubtype::AdjustedBalanceMtd),
            "068" => AmountType::Status(code, AmountSubtype::AdjustedBalanceYtd),
            "070" => AmountType::Status(code, AmountSubtype::ZeroDayFloat),
            "072" => AmountType::Status(code, AmountSubtype::OneDayFloat),
            "073" => AmountType::Status(code, AmountSubtype::FloatAdjustment),
            "074" => AmountType::Status(code, AmountSubtype::TwoOrMoreDaysFloat),
            "075" => AmountType::Status(code, AmountSubtype::ThreeOrMoreDaysFloat),
            "076" => AmountType::Status(code, AmountSubtype::AdjustmentToBalances),
            "077" => AmountType::Status(code, AmountSubtype::AverageAdjustmentToBalancesMtd),
            "078" => AmountType::Status(code, AmountSubtype::AverageAdjustmentToBalancesYtd),
            "079" => AmountType::Status(code, AmountSubtype::FourDayFloat),
            "080" => AmountType::Status(code, AmountSubtype::FiveDayFloat),
            "081" => AmountType::Status(code, AmountSubtype::SixDayFloat),
            "082" => AmountType::Status(code, AmountSubtype::Average1DayFloatMtd),
            "083" => AmountType::Status(code, AmountSubtype::Average1DayFloatYtd),
            "084" => AmountType::Status(code, AmountSubtype::Average2DayFloatMtd),
            "085" => AmountType::Status(code, AmountSubtype::Average2DayFloatYtd),
            "086" => AmountType::Status(code, AmountSubtype::TransferCalculation),
            "100" => AmountType::CreditSummary(code, AmountSubtype::TotalCredits),
            "101" => AmountType::CreditSummary(code, AmountSubtype::TotalCreditAmountMtd),
            "105" => AmountType::CreditSummary(code, AmountSubtype::CreditsNotDetailed),
            "106" => AmountType::CreditSummary(code, AmountSubtype::DepositsSubjectToFloat),
            "107" => AmountType::CreditSummary(code, AmountSubtype::TotalAdjustmentCreditsYtd),
            "109" => AmountType::CreditSummary(code, AmountSubtype::CurrentDayTotalLockboxDeposits),
            "110" => AmountType::CreditSummary(code, AmountSubtype::TotalLockboxDeposits),
            "120" => AmountType::CreditSummary(code, AmountSubtype::EdiTransactionCredit),
            "130" => AmountType::CreditSummary(code, AmountSubtype::TotalConcentrationCredits),
            "131" => AmountType::CreditSummary(code, AmountSubtype::TotalDtcCredits),
            "140" => AmountType::CreditSummary(code, AmountSubtype::TotalAchCredits),
            "146" => AmountType::CreditSummary(code, AmountSubtype::TotalBankCardDeposits),
            "150" => {
                AmountType::CreditSummary(code, AmountSubtype::TotalPreauthorizedPaymentCredits)
            }
            "160" => {
                AmountType::CreditSummary(code, AmountSubtype::TotalAchDisbursingFundingCredits)
            }
            "162" => {
                AmountType::CreditSummary(code, AmountSubtype::CorporateTradePaymentSettlement)
            }
            "163" => AmountType::CreditSummary(code, AmountSubtype::CorporateTradePaymentCredits),
            "167" => AmountType::CreditSummary(code, AmountSubtype::AchSettlementCredits),
            "170" => AmountType::CreditSummary(code, AmountSubtype::TotalOtherCheckDeposits),
            "178" => AmountType::CreditSummary(code, AmountSubtype::ListPostCredits),
            "180" => AmountType::CreditSummary(code, AmountSubtype::TotalLoanProceeds),
            "182" => AmountType::CreditSummary(code, AmountSubtype::TotalBankPreparedDeposits),
            "185" => AmountType::CreditSummary(code, AmountSubtype::TotalMiscellaneousDeposits),
            "186" => AmountType::CreditSummary(code, AmountSubtype::TotalCashLetterCredits),
            "188" => AmountType::CreditSummary(code, AmountSubtype::TotalCashLetterAdjustments),
            "190" => AmountType::CreditSummary(code, AmountSubtype::TotalIncomingMoneyTransfers),
            "200" => AmountType::CreditSummary(code, AmountSubtype::TotalAutomaticTransferCredits),
            "205" => AmountType::CreditSummary(code, AmountSubtype::TotalBookTransferCredits),
            "207" => AmountType::CreditSummary(
                code,
                AmountSubtype::TotalInternationalMoneyTransferCredits,
            ),
            "210" => AmountType::CreditSummary(code, AmountSubtype::TotalInternationalCredits),
            "215" => AmountType::CreditSummary(code, AmountSubtype::TotalLettersOfCredit),
            "230" => AmountType::CreditSummary(code, AmountSubtype::TotalSecurityCredits),
            "231" => AmountType::CreditSummary(code, AmountSubtype::TotalCollectionCredits),
            "239" => AmountType::CreditSummary(code, AmountSubtype::TotalBankersAcceptanceCredits),
            "245" => AmountType::CreditSummary(code, AmountSubtype::MonthlyDividends),
            "250" => AmountType::CreditSummary(code, AmountSubtype::TotalChecksPostedAndReturned),
            "251" => AmountType::CreditSummary(code, AmountSubtype::TotalDebitReversals),
            "256" => AmountType::CreditSummary(code, AmountSubtype::TotalAchReturnItems),
            "260" => AmountType::CreditSummary(code, AmountSubtype::TotalRejectedCredits),
            "270" => AmountType::CreditSummary(code, AmountSubtype::TotalZbaCredits),
            "271" => AmountType::CreditSummary(code, AmountSubtype::NetZeroBalanceAmount),
            "280" => {
                AmountType::CreditSummary(code, AmountSubtype::TotalControlledDisbursingCredits)
            }
            "285" => AmountType::CreditSummary(code, AmountSubtype::TotalDtcDisbursingCredits),
            "294" => AmountType::CreditSummary(code, AmountSubtype::TotalAtmCredits),
            "302" => AmountType::CreditSummary(code, AmountSubtype::CorrespondentBankDeposit),
            "303" => AmountType::CreditSummary(code, AmountSubtype::TotalWireTransfersInFF),
            "304" => AmountType::CreditSummary(code, AmountSubtype::TotalWireTransfersInCHF),
            "305" => AmountType::CreditSummary(code, AmountSubtype::TotalFedFundsSold),
            "307" => AmountType::CreditSummary(code, AmountSubtype::TotalTrustCredits),
            "309" => AmountType::CreditSummary(code, AmountSubtype::TotalValueDatedFunds),
            "310" => AmountType::CreditSummary(code, AmountSubtype::TotalCommercialDeposits),
            "315" => AmountType::CreditSummary(code, AmountSubtype::TotalInternationalCreditsFf),
            "316" => AmountType::CreditSummary(code, AmountSubtype::TotalInternationalCreditsChf),
            "318" => AmountType::CreditSummary(code, AmountSubtype::TotalForeignCheckPurchased),
            "319" => AmountType::CreditSummary(code, AmountSubtype::LateDeposit),
            "320" => AmountType::CreditSummary(code, AmountSubtype::TotalSecuritiesSoldFf),
            "321" => AmountType::CreditSummary(code, AmountSubtype::TotalSecuritiesSoldChf),
            "324" => AmountType::CreditSummary(code, AmountSubtype::TotalSecuritiesMaturedFf),
            "325" => AmountType::CreditSummary(code, AmountSubtype::TotalSecuritiesMaturedChf),
            "326" => AmountType::CreditSummary(code, AmountSubtype::TotalSecuritiesInterest),
            "327" => AmountType::CreditSummary(code, AmountSubtype::TotalSecuritiesMatured),
            "328" => AmountType::CreditSummary(code, AmountSubtype::TotalSecuritiesInterestFf),
            "329" => AmountType::CreditSummary(code, AmountSubtype::TotalSecuritiesInterestChf),
            "330" => AmountType::CreditSummary(code, AmountSubtype::TotalEscrowCredits),
            "332" => AmountType::CreditSummary(
                code,
                AmountSubtype::TotalMiscellaneousSecuritiesCreditsFf,
            ),
            "336" => AmountType::CreditSummary(
                code,
                AmountSubtype::TotalMiscellaneousSecuritiesCreditsChf,
            ),
            "338" => AmountType::CreditSummary(code, AmountSubtype::TotalSecuritiesSold),
            "340" => AmountType::CreditSummary(code, AmountSubtype::TotalBrokerDeposits),
            "341" => AmountType::CreditSummary(code, AmountSubtype::TotalBrokerDepositsFf),
            "343" => AmountType::CreditSummary(code, AmountSubtype::TotalBrokerDepositsChf),
            "350" => AmountType::CreditSummary(code, AmountSubtype::InvestmentSold),
            "352" => AmountType::CreditSummary(code, AmountSubtype::TotalCashCenterCredits),
            "355" => AmountType::CreditSummary(code, AmountSubtype::InvestmentInterest),
            "356" => AmountType::CreditSummary(code, AmountSubtype::TotalCreditAdjustment),
            "360" => AmountType::CreditSummary(
                code,
                AmountSubtype::TotalCreditsLessWireTransferAndReturnedChecks,
            ),
            "361" => AmountType::CreditSummary(
                code,
                AmountSubtype::GrandTotalCreditsLessGrandTotalDebits,
            ),
            "370" => AmountType::CreditSummary(code, AmountSubtype::TotalBackValueCredits),
            "385" => AmountType::CreditSummary(code, AmountSubtype::TotalUniversalCredits),
            "389" => AmountType::CreditSummary(code, AmountSubtype::TotalFreightPaymentCredits),
            "390" => AmountType::CreditSummary(code, AmountSubtype::TotalMiscellaneousCredits),
            "400" => AmountType::DebitSummary(code, AmountSubtype::TotalDebits),
            "401" => AmountType::DebitSummary(code, AmountSubtype::TotalDebitAmountMtd),
            "403" => AmountType::DebitSummary(code, AmountSubtype::TodaysTotalDebits),
            "405" => AmountType::DebitSummary(
                code,
                AmountSubtype::TotalDebitLessWireTransfersAndChargeBacks,
            ),
            "406" => AmountType::DebitSummary(code, AmountSubtype::DebitsNotDetailed),
            "410" => AmountType::DebitSummary(code, AmountSubtype::TotalYtdAdjustment),
            "412" => {
                AmountType::DebitSummary(code, AmountSubtype::TotalDebitsExcludingReturnedItems)
            }
            "416" => AmountType::DebitSummary(code, AmountSubtype::TotalLockboxDebits),
            "420" => AmountType::DebitSummary(code, AmountSubtype::EdiTransactionDebits),
            "430" => AmountType::DebitSummary(code, AmountSubtype::TotalPayableThroughDrafts),
            "446" => {
                AmountType::DebitSummary(code, AmountSubtype::TotalAchDisbursementFundingDebits)
            }
            "450" => AmountType::DebitSummary(code, AmountSubtype::TotalAchDebits),
            "463" => AmountType::DebitSummary(code, AmountSubtype::CorporateTradePaymentDebits),
            "465" => AmountType::DebitSummary(code, AmountSubtype::CorporateTradePaymentSettlement),
            "467" => AmountType::DebitSummary(code, AmountSubtype::AchSettlementDebits),
            "470" => AmountType::DebitSummary(code, AmountSubtype::TotalCheckPaid),
            "471" => AmountType::DebitSummary(code, AmountSubtype::TotalCheckPaidCumulativeMtd),
            "478" => AmountType::DebitSummary(code, AmountSubtype::ListPostDebits),
            "480" => AmountType::DebitSummary(code, AmountSubtype::TotalLoanPayments),
            "482" => AmountType::DebitSummary(code, AmountSubtype::TotalBankOriginatedDebits),
            "486" => AmountType::DebitSummary(code, AmountSubtype::TotalCashLetterDebits),
            "490" => AmountType::DebitSummary(code, AmountSubtype::TotalOutgoingMoneyTransfers),
            "500" => AmountType::DebitSummary(code, AmountSubtype::TotalAutomaticTransferDebits),
            "505" => AmountType::DebitSummary(code, AmountSubtype::TotalBookTransferDebits),
            "507" => {
                AmountType::DebitSummary(code, AmountSubtype::TotalInternationalMoneyTransferDebits)
            }
            "510" => AmountType::DebitSummary(code, AmountSubtype::TotalInternationalDebits),
            "515" => AmountType::DebitSummary(code, AmountSubtype::TotalLettersOfCredit),
            "530" => AmountType::DebitSummary(code, AmountSubtype::TotalSecurityDebits),
            "532" => {
                AmountType::DebitSummary(code, AmountSubtype::TotalAmountOfSecuritiesPurchased)
            }
            "534" => {
                AmountType::DebitSummary(code, AmountSubtype::TotalMiscellaneousSecuritiesDbFf)
            }
            "536" => {
                AmountType::DebitSummary(code, AmountSubtype::TotalMiscellaneousSecuritiesDebitChf)
            }
            "537" => AmountType::DebitSummary(code, AmountSubtype::TotalCollectionDebit),
            "539" => AmountType::DebitSummary(code, AmountSubtype::TotalBankersAcceptancesDebit),
            "550" => AmountType::DebitSummary(code, AmountSubtype::TotalDepositedItemsReturned),
            "551" => AmountType::DebitSummary(code, AmountSubtype::TotalCreditReversals),
            "556" => AmountType::DebitSummary(code, AmountSubtype::TotalAchReturnItems),
            "560" => AmountType::DebitSummary(code, AmountSubtype::TotalRejectedDebits),
            "570" => AmountType::DebitSummary(code, AmountSubtype::TotalZbaDebits),
            "580" => AmountType::DebitSummary(code, AmountSubtype::TotalControlledDisbursingDebits),
            "583" => {
                AmountType::DebitSummary(code, AmountSubtype::TotalDisbursingChecksPaidEarlyAmount)
            }
            "584" => {
                AmountType::DebitSummary(code, AmountSubtype::TotalDisbursingChecksPaidLaterAmount)
            }
            "585" => AmountType::DebitSummary(code, AmountSubtype::DisbursingFundingRequirement),
            "586" => AmountType::DebitSummary(code, AmountSubtype::FrbPresentmentEstimate),
            "587" => AmountType::DebitSummary(code, AmountSubtype::LateDebitsAfterNotification),
            "588" => {
                AmountType::DebitSummary(code, AmountSubtype::TotalDisbursingChecksPaidLastAmount)
            }
            "590" => AmountType::DebitSummary(code, AmountSubtype::TotalDtcDebits),
            "594" => AmountType::DebitSummary(code, AmountSubtype::TotalAtmDebits),
            "596" => AmountType::DebitSummary(code, AmountSubtype::TotalAprDebits),
            "601" => AmountType::DebitSummary(code, AmountSubtype::EstimatedTotalDisbursement),
            "602" => AmountType::DebitSummary(code, AmountSubtype::AdjustedTotalDisbursement),
            "610" => AmountType::DebitSummary(code, AmountSubtype::TotalFundsRequired),
            "611" => AmountType::DebitSummary(code, AmountSubtype::TotalWireTransfersOutChf),
            "612" => AmountType::DebitSummary(code, AmountSubtype::TotalWireTransfersOutFf),
            "613" => AmountType::DebitSummary(code, AmountSubtype::TotalInternationalDebitChf),
            "614" => AmountType::DebitSummary(code, AmountSubtype::TotalInternationalDebitFf),
            "615" => AmountType::DebitSummary(
                code,
                AmountSubtype::TotalFederalReserveBankCommercialBankDebit,
            ),
            "617" => AmountType::DebitSummary(code, AmountSubtype::TotalSecuritiesPurchasedChf),
            "618" => AmountType::DebitSummary(code, AmountSubtype::TotalSecuritiesPurchasedFf),
            "621" => AmountType::DebitSummary(code, AmountSubtype::TotalBrokerDebitsChf),
            "623" => AmountType::DebitSummary(code, AmountSubtype::TotalBrokerDebitsFf),
            "625" => AmountType::DebitSummary(code, AmountSubtype::TotalBrokerDebits),
            "626" => AmountType::DebitSummary(code, AmountSubtype::TotalFedFundsPurchased),
            "628" => AmountType::DebitSummary(code, AmountSubtype::TotalCashCenterDebits),
            "630" => AmountType::DebitSummary(code, AmountSubtype::TotalDebitAdjustments),
            "632" => AmountType::DebitSummary(code, AmountSubtype::TotalTrustDebits),
            "640" => AmountType::DebitSummary(code, AmountSubtype::TotalEscrowDebits),
            "646" => AmountType::DebitSummary(code, AmountSubtype::TransferCalculationDebit),
            "650" => AmountType::DebitSummary(code, AmountSubtype::InvestmentsPurchased),
            "655" => AmountType::DebitSummary(code, AmountSubtype::TotalInvestmentInterestDebits),
            "665" => AmountType::DebitSummary(code, AmountSubtype::InterceptDebits),
            "670" => AmountType::DebitSummary(code, AmountSubtype::TotalBackValueDebits),
            "685" => AmountType::DebitSummary(code, AmountSubtype::TotalUniversalDebits),
            "689" => AmountType::DebitSummary(code, AmountSubtype::FrbFreightPaymentDebits),
            "690" => AmountType::DebitSummary(code, AmountSubtype::TotalMiscellaneousDebits),
            "701" => AmountType::Status(code, AmountSubtype::PrincipalLoanBalance),
            "703" => AmountType::Status(code, AmountSubtype::AvailableCommitmentAmount),
            "705" => AmountType::Status(code, AmountSubtype::PaymentAmountDue),
            "707" => AmountType::Status(code, AmountSubtype::PrincipalAmountPastDue),
            "709" => AmountType::Status(code, AmountSubtype::InterestAmountPastDue),
            "720" => AmountType::CreditSummary(code, AmountSubtype::TotalLoanPayment),
            "760" => AmountType::DebitSummary(code, AmountSubtype::LoanDisbursement),
            other_code => match other_code.parse::<i16>() {
                Ok(n) if n >= 900 && n <= 919 => {
                    AmountType::Status(code, AmountSubtype::CustomStatus)
                }
                Ok(n) if n >= 920 && n <= 959 => {
                    AmountType::CreditSummary(code, AmountSubtype::CustomCreditSummary)
                }
                Ok(n) if n >= 960 && n <= 999 => {
                    AmountType::DebitSummary(code, AmountSubtype::CustomDebitSummary)
                }
                _ => AmountType::Unknown(code, AmountSubtype::Unknown),
            },
        }
    }
}

impl Serialize for AmountType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let (code, type_name, sub_type) = match *self {
            AmountType::Status(ref c, ref t) => (c, "status", t),
            AmountType::CreditSummary(ref c, ref t) => (c, "credit_summary", t),
            AmountType::DebitSummary(ref c, ref t) => (c, "debit_summary", t),
            AmountType::Unknown(ref c, ref t) => (c, "unknown", t),
        };

        let mut state = serializer.serialize_struct("AmountType", 3)?;
        state.serialize_field("code", code)?;
        state.serialize_field("type", type_name)?;
        state.serialize_field("subtype", sub_type)?;
        state.end()
    }
}
