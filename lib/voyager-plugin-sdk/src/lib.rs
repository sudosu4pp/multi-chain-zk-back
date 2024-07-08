use jsonrpsee::{proc_macros::rpc, types::ErrorObjectOwned};
use queue_msg::{optimize::OptimizationResult, Op};
use voyager_message::VoyagerMessage;

/// Defines a single rpc method, voyager_pluginName, which must be implemented by all plugins.
///
/// This name must not contain the `@` character, as it will be used in tag namespacing, in the format `<pluginName>@<tag>`.
#[rpc(client, server, namespace = "voyager")]
trait NamedPlugin {
    #[method(name = "pluginName")]
    fn plugin_name(&self) -> Result<String, ErrorObjectOwned>;
}

/// Run a "pre process" filter on all new messages, returning any messages to be handled by this plugin with a map of key/value pairs.
#[rpc(client, server, namespace = "voyager")]
trait FilterPlugin {
    #[method(name = "filterOps")]
    async fn filter(
        &self,
        msgs: Vec<Op<VoyagerMessage>>,
    ) -> Result<OptimizationResult<VoyagerMessage>, ErrorObjectOwned>;
}

#[rpc(client, server, namespace = "voyager")]
trait ProcessPlugin {
    #[method(name = "processOps")]
    async fn process(
        &self,
        msgs: Vec<Op<VoyagerMessage>>,
    ) -> Result<OptimizationResult<VoyagerMessage>, ErrorObjectOwned>;
}
