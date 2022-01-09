use cosmwasm_std::testing::{mock_env, MockApi, MockStorage};
use cosmwasm_std::{Env, OwnedDeps, Timestamp};

use crate::testing::mock_querier::{mock_dependencies, WasmMockQuerier};

mod executions;
mod mock_querier;
mod queries;

const SHORT_STRING: &str = "a";
const LONG_STRING: &str = "012345678901234567890123456789012345678901234567890123456789012340123456789012345678901234567890123456789012345678901234567890123401234567890123456789012345678901234567890123456789012345678901234012345678900123456789012345678901234567890123456789012345678901234567890123401234567890123456789012345678901234567890123456789012345678901234012345678901234567890123456789012345678901234567890123456789012340123456789012345678901234567890123456789012345678901234567890123401234567890123456789012345678901234567890123456789012345678901234123456789012340123456789012345678901234567890123456789012345678901234567890123401234567890123456789012345678901234567890123456789012345678901234012345678901234567890123456789012345678901234567890123456789012340123456789001234567890123456789012345678901234567890123456789012345678901234012345678901234567890123456789012345678901234567890123456789012340123456789012345678901234567890123456789012345678901234567890123401234567890123456789012345678901234567890123456789012345678901234012345678901234567890123456789012345678901234567890123456789012341234567890123456789012345678901234567890123456789012340123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012340123456789012345678901234567890123456789012345678901234567890123456";

const VOTING_TOKEN: &str = "terra1x46rqay4d3cssq8gxxvqz8xt6nwlz4td20k38v";
const TEST_CREATOR: &str = "terra17lmam6zguazs5q5u6z5mmx76uj63gldnse2pdp";
const TEST_VOTER: &str = "terra1757tkx08n0cqrw7p86ny9lnxsqeth0wgp0em95";
const TEST_VOTER_2: &str = "terra199vw7724lzkwz6lf2hsx04lrxfkz09tg8dlp6r";
const TEST_VOTER_3: &str = "terra18wlvftxzj6zt0xugy2lr9nxzu402690ltaf4ss";
const TEST_TOKEN: &str = "terra1e8ryd9ezefuucd4mje33zdms9m2s90m57878v9";

type MockDeps = OwnedDeps<MockStorage, MockApi, WasmMockQuerier>;

fn mock_deps() -> MockDeps {
    mock_dependencies(&[])
}

fn mock_env_height(height: u64, time: u64) -> Env {
    let mut env = mock_env();
    env.block.height = height;
    env.block.time = Timestamp::from_seconds(time);
    env
}
