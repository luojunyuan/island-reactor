use std::cell::RefCell;

use windows_core::{Interface, Result, implement};

use crate::bindings::*;
use muxc_bindings as Muxc;

#[implement(IXamlMetadataProvider)]
struct IslandApplicationOverrides {
    controls_provider: RefCell<Option<Muxc::XamlControlsXamlMetaDataProvider>>,
}

impl IslandApplicationOverrides {
    fn new() -> Self {
        Self {
            controls_provider: RefCell::new(None),
        }
    }

    fn provider(&self) -> Result<IXamlMetadataProvider> {
        if let Some(provider) = self.controls_provider.borrow().as_ref() {
            return provider.cast();
        }

        let provider = Muxc::XamlControlsXamlMetaDataProvider::new()?;
        *self.controls_provider.borrow_mut() = Some(provider.clone());
        provider.cast()
    }
}

#[allow(non_snake_case)]
impl IXamlMetadataProvider_Impl for IslandApplicationOverrides_Impl {
    fn GetXamlType(&self, r#type: &TypeName) -> Result<IXamlType> {
        self.provider()?.GetXamlType(r#type)
    }

    fn GetXamlTypeByFullName(&self, full_name: &windows_core::HSTRING) -> Result<IXamlType> {
        self.provider()?
            .GetXamlTypeByFullName(&full_name.to_string_lossy())
    }

    fn GetXmlnsDefinitions(&self) -> Result<windows_core::Array<XmlnsDefinition>> {
        self.provider()?.GetXmlnsDefinitions()
    }
}

pub(crate) fn create_island_application() -> Result<Application> {
    Application::compose(IslandApplicationOverrides::new())
}
