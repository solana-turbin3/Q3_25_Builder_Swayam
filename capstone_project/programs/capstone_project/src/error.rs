use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Custom error message")]
    CustomError,
    #[msg("Invalid number of work units")]
    InvalidWorkUnits,
    #[msg("Reviewer is not active")]
    ReviewerNotActive,
    #[msg("No stake found")]
    NoStake,
    #[msg("Already voted on this contribution")]
    AlreadyVoted,
    #[msg("Task already finalized")]
    TaskAlreadyFinalized,
    #[msg("Contribution already finalized")]
    ContributionAlreadyFinalized,
    #[msg("No votes cast")]
    NoVotesCast,
    #[msg("Unauthorized action")]
    Unauthorized,
}
