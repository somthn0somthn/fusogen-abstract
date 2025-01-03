use fusogen::{
    contract::interface::FusogenInterface,
    msg::{
        ConfigResponse, FusogenExecuteMsgFns, FusogenInstantiateMsg, FusogenQueryMsgFns,
        HasClaimedResponse,
    },
    FusogenError, FUSOGEN_NAMESPACE,
};

use abstract_app::objects::{namespace::Namespace, TruncatedChainId};
use abstract_client::{AbstractClient, Application, Environment};
use cosmwasm_std::coins;
use cw_controllers::AdminError;
// Use prelude to get all the necessary imports
use cw_orch::{anyhow, prelude::*};

struct TestEnv<Env: CwEnv> {
    abs: AbstractClient<Env>,
    app: Application<Env, FusogenInterface<Env>>,
}

impl TestEnv<MockBech32> {
    fn setup() -> anyhow::Result<TestEnv<MockBech32>> {
        let mock = MockBech32::new("mock");
        let sender = mock.sender_addr();
        let namespace = Namespace::new(FUSOGEN_NAMESPACE)?;
    
        let abs_client = AbstractClient::builder(mock).build_mock()?;
        abs_client.set_balance(&sender, &coins(123, "ucosm"))?;
    
        let publisher = abs_client
            .account_builder()
            .namespace(namespace)
            .build()?
            .publisher()?;
        publisher.publish_app::<FusogenInterface<_>>()?;
    
        // Install app with dependencies
        let app = publisher.account().install_app_with_dependencies::<FusogenInterface<_>>(
            &FusogenInstantiateMsg {
                source_chain: TruncatedChainId::from_chain_id("juno-1"),
            },
            Empty {},
            &[],
        )?;
    
        Ok(TestEnv {
            abs: abs_client,
            app,
        })
    }
}

#[test]
fn successful_install() -> anyhow::Result<()> {
    let env = TestEnv::setup()?;
    let app = env.app;

    let config = app.config()?;

    assert_eq!(config, ConfigResponse {
        source_chain: TruncatedChainId::from_chain_id("juno-1"),
        destination_chain: TruncatedChainId::from_chain_id("xion-1"),
    });
    
    Ok(())
}

/*
#[test]
fn successful_increment() -> anyhow::Result<()> {
    let env = TestEnv::setup()?;
    let app = env.app;

    app.increment()?;
    let count: CountResponse = app.count()?;
    assert_eq!(count.count, 1);
    Ok(())
}

#[test]
fn successful_reset() -> anyhow::Result<()> {
    let env = TestEnv::setup()?;
    let app = env.app;

    app.reset(42)?;
    let count: CountResponse = app.count()?;
    assert_eq!(count.count, 42);
    Ok(())
}

#[test]
fn failed_reset() -> anyhow::Result<()> {
    let env = TestEnv::setup()?;
    let app = env.app;

    let err: FusogenError = app
        .call_as(&Addr::unchecked("NotAdmin"))
        .reset(9)
        .unwrap_err()
        .downcast()
        .unwrap();
    assert_eq!(err, FusogenError::Admin(AdminError::NotAdmin {}));
    Ok(())
}

#[test]
fn update_config() -> anyhow::Result<()> {
    let env = TestEnv::setup()?;
    let app = env.app;

    app.update_config()?;
    let config = app.config()?;
    let expected_response = fusogen::msg::ConfigResponse {};
    assert_eq!(config, expected_response);
    Ok(())
}

#[test]
fn balance_added() -> anyhow::Result<()> {
    let env = TestEnv::setup()?;
    let account = env.app.account();

    // You can add balance to your account in test environment
    let add_balance = coins(100, "ucosm");
    account.add_balance(&add_balance)?;
    let balances = account.query_balances()?;

    assert_eq!(balances, add_balance);

    // Or set balance to any other address using cw_orch
    let mock_env = env.abs.environment();
    mock_env.add_balance(&env.app.address()?, add_balance.clone())?;
    let balances = mock_env.query_all_balances(&env.app.address()?)?;

    assert_eq!(balances, add_balance);
    Ok(())
}
 */
