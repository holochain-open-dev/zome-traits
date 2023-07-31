# Zome Traits Protocol - Proposal

This is a proposal on how zome traits can be implemented.

## TLDR;

Implement a macro that takes a rust trait with no `&self` methods and defines all its functions as `hdk_extern`.

This is an example on how it would look like:

- Defining a zome trait:

```rust
use zome_traits::zome_trait;

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateTransactionInput {
	agent: AgentPubKey;
	amount: f64;
}

#[zome_trait]
pub trait MutualCreditZomeTrait {
	fn get_balance(agent: AgentPubKey) -> ExternResult<f64>;
	fn create_transaction(input: CreateTransactionInput) -> ExternResult<()>;
}
```

- Implementing a zome trait:

```rust
use zome_traits::zome_trait_extern;
use mutual_credit::MutualCreditZomeTrait;

struct HoloFuel; 

#[zome_trait_extern]
impl MutualCreditZomeTrait for HoloFuel {

	fn get_balance(agent: AgentPubKey) -> ExternResult<f64> {
	  let links =	get_links(agent)?;

		...
	}

	fn create_transaction(input: CreateTransactionInput) -> ExternResult<()> {
	  let my_pub_key = agent_info()?.agent_latest_pubkey;
		create_entry(HoloFuelTransaction {
			sender: my_pub_key,
			recipient: input.agent,
			amount: input.amount
		})?;

		Ok(())
	}
	
}

#[zome_traits_extern] // Defines a `__zome_traits` function
pub enum ZomeTraits {
	MutualCredit(HoloFuel)
}
```

## Goals

- Have well defined interfaces between commonly used types of zomes.
- Enable `aggregator` UIs that can fetch data from different DNAs and display it in the same way.
	- Even if a DNA doesn't include a certain coordinator zome that exposes a certain zome trait, a new coordinator zome could be written for that DNA that implements a that trait, and be added to that DNA dynamically.
- Enable interoperability between DNAs.

## Technical implementation

### zome_trait

Macro that can be placed on a trait that enforces that there is no `&self` parameter in any of the methods of the trait.

### zome_trait_extern

Macro that takes the implementation of a trait and defines all its methods as `#[hdk_extern]`.

### zome_traits_extern

Macro that takes an enum of structs and defines a `__zome_traits` zome function that returns the zome traits that this zome implements, and describes their function signatures with something like ts-rs.