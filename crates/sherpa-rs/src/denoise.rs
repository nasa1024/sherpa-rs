use crate::utils::cstring_from_str;
use crate::get_default_provider;
use eyre::{bail, Result};

#[derive(Debug)]
pub struct Denoise {
    denoiser: *const sherpa_rs_sys::SherpaOnnxOfflineSpeechDenoiser,
}

#[derive(Debug, Clone)]
pub struct DenoiseConfig {
    pub model: String,
    pub provider: Option<String>,
    pub num_threads: Option<i32>,
    pub debug: bool,
}

impl Default for DenoiseConfig {
    fn default() -> Self {
        Self {
            model: String::new(),
            provider: None,
            num_threads: Some(1),
            debug: false,
        }
    }
}

unsafe impl Send for Denoise {}
unsafe impl Sync for Denoise {}

impl Denoise {
    pub fn new(config: DenoiseConfig) -> Result<Self> {
        let debug = config.debug.into();
        let provider = config.provider.unwrap_or(get_default_provider());
        let provider_ptr = cstring_from_str(&provider);
        let num_threads = config.num_threads.unwrap_or(1);
        let model_ptr = cstring_from_str(&config.model);

        let gtcrn_config = sherpa_rs_sys::SherpaOnnxOfflineSpeechDenoiserGtcrnModelConfig {
            model: model_ptr.as_ptr(),
        };

        let model_config = sherpa_rs_sys::SherpaOnnxOfflineSpeechDenoiserModelConfig {
            gtcrn: gtcrn_config,
            num_threads,
            debug,
            provider: provider_ptr.as_ptr(),
        };

        let config = sherpa_rs_sys::SherpaOnnxOfflineSpeechDenoiserConfig {
            model: model_config,
        };

        let denoiser = unsafe { sherpa_rs_sys::SherpaOnnxCreateOfflineSpeechDenoiser(&config) };

        if denoiser.is_null() {
            bail!("Failed to create denoiser");
        }

        Ok(Self { denoiser })
    }

    pub fn run(&self, sample_rate: u32, samples: &[f32]) -> Result<Vec<f32>> {
        unsafe {
            // SherpaOnnxOfflineSpeechDenoiserResult *result =
            //    SherpaOnnxOfflineSpeechDenoiserCompute(denoiser, sample_rate, samples, n);
            let result_ptr = sherpa_rs_sys::SherpaOnnxOfflineSpeechDenoiserCompute(
                self.denoiser,
                sample_rate as i32,
                samples.as_ptr(),
                samples.len().try_into().unwrap(),
            );

            if result_ptr.is_null() {
                bail!("Failed to run denoiser");
            }

            let result = *result_ptr;
            let samples = std::slice::from_raw_parts(result.samples, result.num_samples as usize).to_vec();
            
            // Destroy result
            sherpa_rs_sys::SherpaOnnxOfflineSpeechDenoiserDestroyResult(result_ptr);

            Ok(samples)
        }
    }
}

impl Drop for Denoise {
    fn drop(&mut self) {
        unsafe {
            sherpa_rs_sys::SherpaOnnxDestroyOfflineSpeechDenoiser(self.denoiser);
        }
    }
}
