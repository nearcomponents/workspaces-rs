use crate::types::{AccountId, InMemorySigner};

pub struct Account {
    pub(crate) id: AccountId,
    pub(crate) signer: InMemorySigner,
}

impl Account {
    pub(crate) fn new(id: AccountId, signer: InMemorySigner) -> Self {
        Self { id, signer }
    }

    pub fn id(&self) -> &AccountId {
        &self.id
    }

    pub fn signer(&self) -> &InMemorySigner {
        &self.signer
    }
}

// TODO: allow users to create Contracts so that they can call into
// them without deploying the contract themselves.
pub struct Contract {
    pub(crate) account: Account,
}

impl Contract {
    pub(crate) fn new(id: AccountId, signer: InMemorySigner) -> Self {
        Self {
            account: Account::new(id, signer),
        }
    }

    pub(crate) fn account(account: Account) -> Self {
        Self { account }
    }

    pub fn id(&self) -> &AccountId {
        &self.account.id
    }

    pub(crate) fn signer(&self) -> &InMemorySigner {
        self.account.signer()
    }
}
