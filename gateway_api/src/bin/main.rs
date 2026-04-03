use fw_base::pass::gw_pass::AuthInfoPassStrategyEnum;
use fw_base::set_gw_dispatch_val;
use fw_boot::ext::RunConfigExt;
use fw_error::FwResult;
use fw_gateway::run::GatewayRunner;
use gateway_api::config::StaticConfig;
use gateway_api::hook::HttpRequestHook;

fn main() -> FwResult<()> {
    let (runner, router) = GatewayRunner::new()?;

    let rs = runner.get_rs();

    let config_group = runner
        .get_app()
        .get_cfg()
        .nacos_center_cfg
        .config
        .group_name
        .clone();

    let static_cfg = runner.execute(move || async move {
        let static_cfg = rs
            .nacos_proxy()
            .get_nacos_configure()
            .fetch_static_config::<StaticConfig>(&config_group)
            .await?;

        Ok(static_cfg)
    })?;

    let strategy = static_cfg
        .comm_static_cfg
        .gw_dispatch_cfg
        .pass_strategy
        .clone()
        .unwrap_or("".to_string());

    let pass_strategy = AuthInfoPassStrategyEnum::new(&strategy);
    let hook_ext = HttpRequestHook::new(pass_strategy);
    runner.run(static_cfg, router, hook_ext)?;

    Ok(())
}
