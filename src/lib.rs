use obs_wrapper::{
    prelude::*,
    source::*,
    properties::*,
    graphics::*,
    obs_register_module,
    obs_string,
};

use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;
use std::ffi::CString;

struct Shader {
    source: SourceContext,
    effect: GraphicsEffect,
}

impl Shader {
    fn load_shader_file() -> Result<GraphicsEffect, io::Error> {
        let path = Path::new("C:\\projects\\obs-shader-plugin\\src\\test_shader.effect");
        let mut file = File::open(&path)?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;
        let cs = CString::new(buf.as_bytes())?;
        let shader_string = ObsString::Dynamic(cs);

        if let Some(effect) = GraphicsEffect::from_effect_string(
            shader_string,
            obs_string!("test_shader.effect"),
        ) {
            Ok(effect)
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "Failed to call GraphicsEffect::from_effect_string()"))
        }
    }
}

impl Sourceable for Shader {
    fn get_id() -> ObsString {
        obs_string!("shader")
    }

    fn get_type() -> SourceType {
        SourceType::FILTER
    }

    fn create(_create: &mut CreatableSourceContext<Self>, source: SourceContext) -> Self {
        let effect = Shader::load_shader_file().unwrap_or_else(|err| {
            panic!("Could not load shader file: {}", err)
        });
        Self {
            source,
            effect,
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

impl VideoRenderSource for Shader {
    fn video_render(&mut self, _context: &mut GlobalContext, render: &mut VideoRenderContext) {
        let effect = &mut self.effect;
        let source = &mut self.source;

        let mut target_cx: u32 = 1;
        let mut target_cy: u32 = 1;

        source.do_with_target(|target| {
            target_cx = target.get_base_width();
            target_cy = target.get_base_height();
        });

        source.process_filter_tech(
            render,
            effect,
            (target_cx, target_cy),
            GraphicsColorFormat::RGBA,
            GraphicsAllowDirectRendering::NoDirectRendering,
            obs_string!("DrawMultiply"),
            |context, _effect| {
            },
        );
    }
}

impl VideoTickSource for Shader {
    fn video_tick(&mut self, _seconds: f32) {
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
            .enable_video_render()
            .enable_video_tick()
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
