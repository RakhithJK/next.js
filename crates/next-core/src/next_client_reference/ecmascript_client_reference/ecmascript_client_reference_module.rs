#![allow(rustdoc::private_intra_doc_links)]
use anyhow::{bail, Result};
use turbo_tasks::{RcStr, ResolvedVc, Vc};
use turbopack_core::{
    asset::{Asset, AssetContent},
    ident::AssetIdent,
    module::{Module, Modules},
};
use turbopack_ecmascript::chunk::EcmascriptChunkPlaceable;

/// A marker module used by the
/// [super::ecmascript_client_reference_proxy_module::EcmascriptClientReferenceProxyModule] to
/// indicate which client reference should appear in the client reference manifest.
#[turbo_tasks::value]
pub struct EcmascriptClientReferenceModule {
    pub server_ident: Vc<AssetIdent>,
    pub client_module: ResolvedVc<Box<dyn EcmascriptChunkPlaceable>>,
    pub ssr_module: ResolvedVc<Box<dyn EcmascriptChunkPlaceable>>,
}

#[turbo_tasks::value_impl]
impl EcmascriptClientReferenceModule {
    /// Create a new [EcmascriptClientReferenceModule].
    ///
    /// # Arguments
    ///
    /// * `server_ident` - The identifier of the server module, used to identify the client
    ///   reference.
    /// * `client_module` - The client module.
    /// * `ssr_module` - The SSR module.
    #[turbo_tasks::function]
    pub fn new(
        server_ident: Vc<AssetIdent>,
        client_module: ResolvedVc<Box<dyn EcmascriptChunkPlaceable>>,
        ssr_module: ResolvedVc<Box<dyn EcmascriptChunkPlaceable>>,
    ) -> Vc<EcmascriptClientReferenceModule> {
        EcmascriptClientReferenceModule {
            server_ident,
            client_module,
            ssr_module,
        }
        .cell()
    }
}

#[turbo_tasks::function]
fn ecmascript_client_reference_modifier() -> Vc<RcStr> {
    Vc::cell("ecmascript client reference".into())
}

#[turbo_tasks::value_impl]
impl Module for EcmascriptClientReferenceModule {
    #[turbo_tasks::function]
    fn ident(&self) -> Vc<AssetIdent> {
        self.server_ident
            .with_modifier(ecmascript_client_reference_modifier())
    }

    #[turbo_tasks::function]
    fn additional_layers_modules(&self) -> Vc<Modules> {
        let client_module = ResolvedVc::upcast(self.client_module);
        let ssr_module = ResolvedVc::upcast(self.ssr_module);
        Vc::cell(vec![client_module, ssr_module])
    }
}

#[turbo_tasks::value_impl]
impl Asset for EcmascriptClientReferenceModule {
    #[turbo_tasks::function]
    fn content(&self) -> Result<Vc<AssetContent>> {
        // The ES client reference asset only serves as a marker asset.
        bail!("EcmascriptClientReferenceModule has no content")
    }
}
