import { GeneratedType } from "@cosmjs/proto-signing";
import { MsgLiquidateLoan } from "./types/loan/loan/tx";
import { MsgCancelLoan } from "./types/loan/loan/tx";
import { MsgRequestLoan } from "./types/loan/loan/tx";
import { MsgRepayLoan } from "./types/loan/loan/tx";
import { MsgApproveLoan } from "./types/loan/loan/tx";

const msgTypes: Array<[string, GeneratedType]>  = [
    ["/loan.loan.MsgLiquidateLoan", MsgLiquidateLoan],
    ["/loan.loan.MsgCancelLoan", MsgCancelLoan],
    ["/loan.loan.MsgRequestLoan", MsgRequestLoan],
    ["/loan.loan.MsgRepayLoan", MsgRepayLoan],
    ["/loan.loan.MsgApproveLoan", MsgApproveLoan],
    
];

export { msgTypes }