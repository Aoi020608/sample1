package keeper

import (
	"context"

	"loan/x/loan/types"

	sdk "github.com/cosmos/cosmos-sdk/types"
)

func (k msgServer) RequestLoan(goCtx context.Context, msg *types.MsgRequestLoan) (*types.MsgRequestLoanResponse, error) {
	ctx := sdk.UnwrapSDKContext(goCtx)

	// Create a new loan
	var loan = types.Loan{
		Amount:     msg.Amount,
		Fee:        msg.Fee,
		Collateral: msg.Collateral,
		Deadline:   msg.Deadline,
		State:      "requested",
		Borrower:   msg.Creator,
	}

	// Get the borrowe address
	borrower, _ := sdk.AccAddressFromBech32(msg.Creator)

	// Get the borrowe address
	collateral, err := sdk.ParseCoinsNormalized(loan.Collateral)
	if err != nil {
		panic(err)
	}

	// Use the module account as escrow account
	sdkError := k.bankKeeper.SendCoinsFromAccountToModule(ctx, borrower, types.ModuleName, collateral)
	if sdkError != nil {
		return nil, sdkError
	}

	k.AppendLoan(ctx, loan)

	return &types.MsgRequestLoanResponse{}, nil
}
