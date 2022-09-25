use obs_wrapper::{
    prelude::*,
    source::*,
    properties::*,
    obs_register_module,
    obs_string,
};

struct Shader;

impl Sourceable for Shader {
    fn get_id() -> ObsString {
        obs_string!("shader")
    }

    fn get_type() -> SourceType {
        SourceType::FILTER
    }

    fn create(_create: &mut CreatableSourceContext<Self>, _source: SourceContext) -> Self {
        Self {
        }
    }
}

impl GetNameSource for Shader {
    fn get_name() -> ObsString {
        obs_string!("Shader")
    }
}

impl GetDefaultsSource for Shader {
    fn get_defaults(_settings: &mut DataObj) {
    }
}

impl GetPropertiesSource for Shader {
    fn get_properties(&mut self) -> Properties {
        let properties = Properties::new();
        properties
    }
}

impl UpdateSource for Shader {
    fn update(&mut self, _settings: &mut DataObj, _context: &mut GlobalContext) {
    }
}

struct ShaderModule {
    context: ModuleContext
}

impl Module for ShaderModule {
    fn new(context: ModuleContext) -> Self {
        Self { context }
    }

    fn get_ctx(&self) -> &ModuleContext {
        &self.context
    }

    fn load(&mut self, load_context: &mut LoadContext) -> bool {
        let source = load_context
            .create_source_builder::<Shader>()
            .enable_get_name()
            .enable_get_defaults()
            .enable_get_properties()
            .enable_update()
            .build();

        load_context.register_source(source);

        true
    }

    fn description() -> ObsString {
        obs_string!("Apply shader to video")
    }

    fn name() -> ObsString {
        obs_string!("Shader")
    }

    fn author() -> ObsString {
        obs_string!("Yoshinori Sano")
    }
}

obs_register_module!(ShaderModule);
