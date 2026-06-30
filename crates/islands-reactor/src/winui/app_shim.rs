use std::cell::RefCell;

use windows_core::{Interface, Result, implement};

use crate::bindings::*;
use crate::bindings_iuxc as Iuxc;
use crate::bindings_muxc as Muxc;

#[implement(IXamlMetadataProvider)]
struct IslandApplicationOverrides {
    muxc_provider: RefCell<Option<Muxc::XamlControlsXamlMetaDataProvider>>,
    iuxc_provider: RefCell<Option<Iuxc::XamlMetaDataProvider>>,
}

impl IslandApplicationOverrides {
    fn new() -> Self {
        Self {
            muxc_provider: RefCell::new(None),
            iuxc_provider: RefCell::new(None),
        }
    }

    fn muxc_provider(&self) -> Result<IXamlMetadataProvider> {
        if let Some(provider) = self.muxc_provider.borrow().as_ref() {
            return provider.cast();
        }

        let provider = Muxc::XamlControlsXamlMetaDataProvider::new()?;
        *self.muxc_provider.borrow_mut() = Some(provider.clone());
        provider.cast()
    }

    fn iuxc_provider(&self) -> Result<IXamlMetadataProvider> {
        if let Some(provider) = self.iuxc_provider.borrow().as_ref() {
            return provider.cast();
        }

        let provider = Iuxc::XamlMetaDataProvider::new()?;
        *self.iuxc_provider.borrow_mut() = Some(provider.clone());
        provider.cast()
    }

    fn provider_for_name(&self, name: &str) -> Result<IXamlMetadataProvider> {
        if name.starts_with("Islands.UI.Xaml") {
            self.iuxc_provider()
        } else {
            self.muxc_provider()
        }
    }
}

#[allow(non_snake_case)]
impl IXamlMetadataProvider_Impl for IslandApplicationOverrides_Impl {
    fn GetXamlType(&self, r#type: &TypeName) -> Result<IXamlType> {
        let name = r#type.Name.to_string_lossy();
        self.provider_for_name(&name)?.GetXamlType(r#type)
    }

    fn GetXamlTypeByFullName(&self, full_name: &windows_core::HSTRING) -> Result<IXamlType> {
        let full_name = full_name.to_string_lossy();
        self.provider_for_name(&full_name)?
            .GetXamlTypeByFullName(&full_name)
    }

    fn GetXmlnsDefinitions(&self) -> Result<windows_core::Array<XmlnsDefinition>> {
        let muxc = self.muxc_provider()?.GetXmlnsDefinitions()?;
        let iuxc = self.iuxc_provider()?.GetXmlnsDefinitions()?;
        let mut definitions = Vec::with_capacity(muxc.len() + iuxc.len());
        definitions.extend_from_slice(&muxc);
        definitions.extend_from_slice(&iuxc);
        Ok(windows_core::Array::from_slice(&definitions))
    }
}

pub(crate) fn create_island_application() -> Result<Application> {
    Application::compose(IslandApplicationOverrides::new())
}
