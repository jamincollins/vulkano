// This file is auto-generated by vulkano autogen from vk.xml header version 1.3.281.
// It should not be edited manually. Changes should be made by editing autogen.


/**Raw Vulkan global entry point-level functions.

To use these, you need to include the Ash crate, using the same version Vulkano uses.*/
#[allow(missing_docs)]
pub struct EntryFunctions {
    pub v1_0: ash::EntryFnV1_0,
    pub v1_1: ash::EntryFnV1_1,
    pub _ne: crate::NonExhaustive,
}
impl EntryFunctions {
    pub(crate) fn load<F>(mut load_fn: F) -> EntryFunctions
    where
        F: FnMut(&CStr) -> *const c_void,
    {
        EntryFunctions {
            v1_0: ash::EntryFnV1_0::load(&mut load_fn),
            v1_1: ash::EntryFnV1_1::load(&mut load_fn),
            _ne: crate::NonExhaustive(()),
        }
    }
}
impl std::fmt::Debug for EntryFunctions {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_struct(stringify!(EntryFunctions)).finish_non_exhaustive()
    }
}
/**Raw Vulkan instance-level functions.

To use these, you need to include the Ash crate, using the same version Vulkano uses.*/
#[allow(missing_docs)]
pub struct InstanceFunctions {
    pub v1_0: ash::InstanceFnV1_0,
    pub v1_1: ash::InstanceFnV1_1,
    pub v1_3: ash::InstanceFnV1_3,
    pub khr_android_surface: ash::khr::android_surface::InstanceFn,
    pub khr_calibrated_timestamps: ash::khr::calibrated_timestamps::InstanceFn,
    pub khr_cooperative_matrix: ash::khr::cooperative_matrix::InstanceFn,
    pub khr_device_group: ash::khr::device_group::InstanceFn,
    pub khr_device_group_creation: ash::khr::device_group_creation::InstanceFn,
    pub khr_display: ash::khr::display::InstanceFn,
    pub khr_external_fence_capabilities: ash::khr::external_fence_capabilities::InstanceFn,
    pub khr_external_memory_capabilities: ash::khr::external_memory_capabilities::InstanceFn,
    pub khr_external_semaphore_capabilities: ash::khr::external_semaphore_capabilities::InstanceFn,
    pub khr_fragment_shading_rate: ash::khr::fragment_shading_rate::InstanceFn,
    pub khr_get_display_properties2: ash::khr::get_display_properties2::InstanceFn,
    pub khr_get_physical_device_properties2: ash::khr::get_physical_device_properties2::InstanceFn,
    pub khr_get_surface_capabilities2: ash::khr::get_surface_capabilities2::InstanceFn,
    pub khr_performance_query: ash::khr::performance_query::InstanceFn,
    pub khr_surface: ash::khr::surface::InstanceFn,
    pub khr_swapchain: ash::khr::swapchain::InstanceFn,
    pub khr_video_encode_queue: ash::khr::video_encode_queue::InstanceFn,
    pub khr_video_queue: ash::khr::video_queue::InstanceFn,
    pub khr_wayland_surface: ash::khr::wayland_surface::InstanceFn,
    pub khr_win32_surface: ash::khr::win32_surface::InstanceFn,
    pub khr_xcb_surface: ash::khr::xcb_surface::InstanceFn,
    pub khr_xlib_surface: ash::khr::xlib_surface::InstanceFn,
    pub ext_acquire_drm_display: ash::ext::acquire_drm_display::InstanceFn,
    pub ext_acquire_xlib_display: ash::ext::acquire_xlib_display::InstanceFn,
    pub ext_calibrated_timestamps: ash::ext::calibrated_timestamps::InstanceFn,
    pub ext_debug_report: ash::ext::debug_report::InstanceFn,
    pub ext_debug_utils: ash::ext::debug_utils::InstanceFn,
    pub ext_direct_mode_display: ash::ext::direct_mode_display::InstanceFn,
    pub ext_directfb_surface: ash::ext::directfb_surface::InstanceFn,
    pub ext_display_surface_counter: ash::ext::display_surface_counter::InstanceFn,
    pub ext_full_screen_exclusive: ash::ext::full_screen_exclusive::InstanceFn,
    pub ext_headless_surface: ash::ext::headless_surface::InstanceFn,
    pub ext_metal_surface: ash::ext::metal_surface::InstanceFn,
    pub ext_sample_locations: ash::ext::sample_locations::InstanceFn,
    pub ext_tooling_info: ash::ext::tooling_info::InstanceFn,
    pub fuchsia_imagepipe_surface: ash::fuchsia::imagepipe_surface::InstanceFn,
    pub ggp_stream_descriptor_surface: ash::ggp::stream_descriptor_surface::InstanceFn,
    pub mvk_ios_surface: ash::mvk::ios_surface::InstanceFn,
    pub mvk_macos_surface: ash::mvk::macos_surface::InstanceFn,
    pub nn_vi_surface: ash::nn::vi_surface::InstanceFn,
    pub nv_acquire_winrt_display: ash::nv::acquire_winrt_display::InstanceFn,
    pub nv_cooperative_matrix: ash::nv::cooperative_matrix::InstanceFn,
    pub nv_coverage_reduction_mode: ash::nv::coverage_reduction_mode::InstanceFn,
    pub nv_external_memory_capabilities: ash::nv::external_memory_capabilities::InstanceFn,
    pub nv_optical_flow: ash::nv::optical_flow::InstanceFn,
    pub qnx_screen_surface: ash::qnx::screen_surface::InstanceFn,
    pub _ne: crate::NonExhaustive,
}
impl InstanceFunctions {
    pub(crate) fn load<F>(mut load_fn: F) -> InstanceFunctions
    where
        F: FnMut(&CStr) -> *const c_void,
    {
        InstanceFunctions {
            v1_0: ash::InstanceFnV1_0::load(&mut load_fn),
            v1_1: ash::InstanceFnV1_1::load(&mut load_fn),
            v1_3: ash::InstanceFnV1_3::load(&mut load_fn),
            khr_android_surface: ash::khr::android_surface::InstanceFn::load(
                &mut load_fn,
            ),
            khr_calibrated_timestamps: ash::khr::calibrated_timestamps::InstanceFn::load(
                &mut load_fn,
            ),
            khr_cooperative_matrix: ash::khr::cooperative_matrix::InstanceFn::load(
                &mut load_fn,
            ),
            khr_device_group: ash::khr::device_group::InstanceFn::load(&mut load_fn),
            khr_device_group_creation: ash::khr::device_group_creation::InstanceFn::load(
                &mut load_fn,
            ),
            khr_display: ash::khr::display::InstanceFn::load(&mut load_fn),
            khr_external_fence_capabilities: ash::khr::external_fence_capabilities::InstanceFn::load(
                &mut load_fn,
            ),
            khr_external_memory_capabilities: ash::khr::external_memory_capabilities::InstanceFn::load(
                &mut load_fn,
            ),
            khr_external_semaphore_capabilities: ash::khr::external_semaphore_capabilities::InstanceFn::load(
                &mut load_fn,
            ),
            khr_fragment_shading_rate: ash::khr::fragment_shading_rate::InstanceFn::load(
                &mut load_fn,
            ),
            khr_get_display_properties2: ash::khr::get_display_properties2::InstanceFn::load(
                &mut load_fn,
            ),
            khr_get_physical_device_properties2: ash::khr::get_physical_device_properties2::InstanceFn::load(
                &mut load_fn,
            ),
            khr_get_surface_capabilities2: ash::khr::get_surface_capabilities2::InstanceFn::load(
                &mut load_fn,
            ),
            khr_performance_query: ash::khr::performance_query::InstanceFn::load(
                &mut load_fn,
            ),
            khr_surface: ash::khr::surface::InstanceFn::load(&mut load_fn),
            khr_swapchain: ash::khr::swapchain::InstanceFn::load(&mut load_fn),
            khr_video_encode_queue: ash::khr::video_encode_queue::InstanceFn::load(
                &mut load_fn,
            ),
            khr_video_queue: ash::khr::video_queue::InstanceFn::load(&mut load_fn),
            khr_wayland_surface: ash::khr::wayland_surface::InstanceFn::load(
                &mut load_fn,
            ),
            khr_win32_surface: ash::khr::win32_surface::InstanceFn::load(&mut load_fn),
            khr_xcb_surface: ash::khr::xcb_surface::InstanceFn::load(&mut load_fn),
            khr_xlib_surface: ash::khr::xlib_surface::InstanceFn::load(&mut load_fn),
            ext_acquire_drm_display: ash::ext::acquire_drm_display::InstanceFn::load(
                &mut load_fn,
            ),
            ext_acquire_xlib_display: ash::ext::acquire_xlib_display::InstanceFn::load(
                &mut load_fn,
            ),
            ext_calibrated_timestamps: ash::ext::calibrated_timestamps::InstanceFn::load(
                &mut load_fn,
            ),
            ext_debug_report: ash::ext::debug_report::InstanceFn::load(&mut load_fn),
            ext_debug_utils: ash::ext::debug_utils::InstanceFn::load(&mut load_fn),
            ext_direct_mode_display: ash::ext::direct_mode_display::InstanceFn::load(
                &mut load_fn,
            ),
            ext_directfb_surface: ash::ext::directfb_surface::InstanceFn::load(
                &mut load_fn,
            ),
            ext_display_surface_counter: ash::ext::display_surface_counter::InstanceFn::load(
                &mut load_fn,
            ),
            ext_full_screen_exclusive: ash::ext::full_screen_exclusive::InstanceFn::load(
                &mut load_fn,
            ),
            ext_headless_surface: ash::ext::headless_surface::InstanceFn::load(
                &mut load_fn,
            ),
            ext_metal_surface: ash::ext::metal_surface::InstanceFn::load(&mut load_fn),
            ext_sample_locations: ash::ext::sample_locations::InstanceFn::load(
                &mut load_fn,
            ),
            ext_tooling_info: ash::ext::tooling_info::InstanceFn::load(&mut load_fn),
            fuchsia_imagepipe_surface: ash::fuchsia::imagepipe_surface::InstanceFn::load(
                &mut load_fn,
            ),
            ggp_stream_descriptor_surface: ash::ggp::stream_descriptor_surface::InstanceFn::load(
                &mut load_fn,
            ),
            mvk_ios_surface: ash::mvk::ios_surface::InstanceFn::load(&mut load_fn),
            mvk_macos_surface: ash::mvk::macos_surface::InstanceFn::load(&mut load_fn),
            nn_vi_surface: ash::nn::vi_surface::InstanceFn::load(&mut load_fn),
            nv_acquire_winrt_display: ash::nv::acquire_winrt_display::InstanceFn::load(
                &mut load_fn,
            ),
            nv_cooperative_matrix: ash::nv::cooperative_matrix::InstanceFn::load(
                &mut load_fn,
            ),
            nv_coverage_reduction_mode: ash::nv::coverage_reduction_mode::InstanceFn::load(
                &mut load_fn,
            ),
            nv_external_memory_capabilities: ash::nv::external_memory_capabilities::InstanceFn::load(
                &mut load_fn,
            ),
            nv_optical_flow: ash::nv::optical_flow::InstanceFn::load(&mut load_fn),
            qnx_screen_surface: ash::qnx::screen_surface::InstanceFn::load(&mut load_fn),
            _ne: crate::NonExhaustive(()),
        }
    }
}
impl std::fmt::Debug for InstanceFunctions {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_struct(stringify!(InstanceFunctions)).finish_non_exhaustive()
    }
}
/**Raw Vulkan device-level functions.

To use these, you need to include the Ash crate, using the same version Vulkano uses.*/
#[allow(missing_docs)]
pub struct DeviceFunctions {
    pub v1_0: ash::DeviceFnV1_0,
    pub v1_1: ash::DeviceFnV1_1,
    pub v1_2: ash::DeviceFnV1_2,
    pub v1_3: ash::DeviceFnV1_3,
    pub khr_acceleration_structure: ash::khr::acceleration_structure::DeviceFn,
    pub khr_bind_memory2: ash::khr::bind_memory2::DeviceFn,
    pub khr_buffer_device_address: ash::khr::buffer_device_address::DeviceFn,
    pub khr_calibrated_timestamps: ash::khr::calibrated_timestamps::DeviceFn,
    pub khr_copy_commands2: ash::khr::copy_commands2::DeviceFn,
    pub khr_create_renderpass2: ash::khr::create_renderpass2::DeviceFn,
    pub khr_deferred_host_operations: ash::khr::deferred_host_operations::DeviceFn,
    pub khr_descriptor_update_template: ash::khr::descriptor_update_template::DeviceFn,
    pub khr_device_group: ash::khr::device_group::DeviceFn,
    pub khr_display_swapchain: ash::khr::display_swapchain::DeviceFn,
    pub khr_draw_indirect_count: ash::khr::draw_indirect_count::DeviceFn,
    pub khr_dynamic_rendering: ash::khr::dynamic_rendering::DeviceFn,
    pub khr_dynamic_rendering_local_read: ash::khr::dynamic_rendering_local_read::DeviceFn,
    pub khr_external_fence_fd: ash::khr::external_fence_fd::DeviceFn,
    pub khr_external_fence_win32: ash::khr::external_fence_win32::DeviceFn,
    pub khr_external_memory_fd: ash::khr::external_memory_fd::DeviceFn,
    pub khr_external_memory_win32: ash::khr::external_memory_win32::DeviceFn,
    pub khr_external_semaphore_fd: ash::khr::external_semaphore_fd::DeviceFn,
    pub khr_external_semaphore_win32: ash::khr::external_semaphore_win32::DeviceFn,
    pub khr_fragment_shading_rate: ash::khr::fragment_shading_rate::DeviceFn,
    pub khr_get_memory_requirements2: ash::khr::get_memory_requirements2::DeviceFn,
    pub khr_line_rasterization: ash::khr::line_rasterization::DeviceFn,
    pub khr_maintenance1: ash::khr::maintenance1::DeviceFn,
    pub khr_maintenance3: ash::khr::maintenance3::DeviceFn,
    pub khr_maintenance4: ash::khr::maintenance4::DeviceFn,
    pub khr_maintenance5: ash::khr::maintenance5::DeviceFn,
    pub khr_maintenance6: ash::khr::maintenance6::DeviceFn,
    pub khr_map_memory2: ash::khr::map_memory2::DeviceFn,
    pub khr_performance_query: ash::khr::performance_query::DeviceFn,
    pub khr_pipeline_executable_properties: ash::khr::pipeline_executable_properties::DeviceFn,
    pub khr_present_wait: ash::khr::present_wait::DeviceFn,
    pub khr_push_descriptor: ash::khr::push_descriptor::DeviceFn,
    pub khr_ray_tracing_maintenance1: ash::khr::ray_tracing_maintenance1::DeviceFn,
    pub khr_ray_tracing_pipeline: ash::khr::ray_tracing_pipeline::DeviceFn,
    pub khr_sampler_ycbcr_conversion: ash::khr::sampler_ycbcr_conversion::DeviceFn,
    pub khr_shared_presentable_image: ash::khr::shared_presentable_image::DeviceFn,
    pub khr_swapchain: ash::khr::swapchain::DeviceFn,
    pub khr_synchronization2: ash::khr::synchronization2::DeviceFn,
    pub khr_timeline_semaphore: ash::khr::timeline_semaphore::DeviceFn,
    pub khr_video_decode_queue: ash::khr::video_decode_queue::DeviceFn,
    pub khr_video_encode_queue: ash::khr::video_encode_queue::DeviceFn,
    pub khr_video_queue: ash::khr::video_queue::DeviceFn,
    pub ext_attachment_feedback_loop_dynamic_state: ash::ext::attachment_feedback_loop_dynamic_state::DeviceFn,
    pub ext_buffer_device_address: ash::ext::buffer_device_address::DeviceFn,
    pub ext_calibrated_timestamps: ash::ext::calibrated_timestamps::DeviceFn,
    pub ext_color_write_enable: ash::ext::color_write_enable::DeviceFn,
    pub ext_conditional_rendering: ash::ext::conditional_rendering::DeviceFn,
    pub ext_debug_marker: ash::ext::debug_marker::DeviceFn,
    pub ext_debug_utils: ash::ext::debug_utils::DeviceFn,
    pub ext_depth_bias_control: ash::ext::depth_bias_control::DeviceFn,
    pub ext_descriptor_buffer: ash::ext::descriptor_buffer::DeviceFn,
    pub ext_device_fault: ash::ext::device_fault::DeviceFn,
    pub ext_discard_rectangles: ash::ext::discard_rectangles::DeviceFn,
    pub ext_display_control: ash::ext::display_control::DeviceFn,
    pub ext_extended_dynamic_state: ash::ext::extended_dynamic_state::DeviceFn,
    pub ext_extended_dynamic_state2: ash::ext::extended_dynamic_state2::DeviceFn,
    pub ext_extended_dynamic_state3: ash::ext::extended_dynamic_state3::DeviceFn,
    pub ext_external_memory_host: ash::ext::external_memory_host::DeviceFn,
    pub ext_full_screen_exclusive: ash::ext::full_screen_exclusive::DeviceFn,
    pub ext_hdr_metadata: ash::ext::hdr_metadata::DeviceFn,
    pub ext_host_image_copy: ash::ext::host_image_copy::DeviceFn,
    pub ext_host_query_reset: ash::ext::host_query_reset::DeviceFn,
    pub ext_image_compression_control: ash::ext::image_compression_control::DeviceFn,
    pub ext_image_drm_format_modifier: ash::ext::image_drm_format_modifier::DeviceFn,
    pub ext_line_rasterization: ash::ext::line_rasterization::DeviceFn,
    pub ext_mesh_shader: ash::ext::mesh_shader::DeviceFn,
    pub ext_metal_objects: ash::ext::metal_objects::DeviceFn,
    pub ext_multi_draw: ash::ext::multi_draw::DeviceFn,
    pub ext_opacity_micromap: ash::ext::opacity_micromap::DeviceFn,
    pub ext_pageable_device_local_memory: ash::ext::pageable_device_local_memory::DeviceFn,
    pub ext_pipeline_properties: ash::ext::pipeline_properties::DeviceFn,
    pub ext_private_data: ash::ext::private_data::DeviceFn,
    pub ext_sample_locations: ash::ext::sample_locations::DeviceFn,
    pub ext_shader_module_identifier: ash::ext::shader_module_identifier::DeviceFn,
    pub ext_shader_object: ash::ext::shader_object::DeviceFn,
    pub ext_swapchain_maintenance1: ash::ext::swapchain_maintenance1::DeviceFn,
    pub ext_transform_feedback: ash::ext::transform_feedback::DeviceFn,
    pub ext_validation_cache: ash::ext::validation_cache::DeviceFn,
    pub ext_vertex_input_dynamic_state: ash::ext::vertex_input_dynamic_state::DeviceFn,
    pub amdx_shader_enqueue: ash::amdx::shader_enqueue::DeviceFn,
    pub amd_buffer_marker: ash::amd::buffer_marker::DeviceFn,
    pub amd_display_native_hdr: ash::amd::display_native_hdr::DeviceFn,
    pub amd_draw_indirect_count: ash::amd::draw_indirect_count::DeviceFn,
    pub amd_shader_info: ash::amd::shader_info::DeviceFn,
    pub android_external_memory_android_hardware_buffer: ash::android::external_memory_android_hardware_buffer::DeviceFn,
    pub fuchsia_buffer_collection: ash::fuchsia::buffer_collection::DeviceFn,
    pub fuchsia_external_memory: ash::fuchsia::external_memory::DeviceFn,
    pub fuchsia_external_semaphore: ash::fuchsia::external_semaphore::DeviceFn,
    pub google_display_timing: ash::google::display_timing::DeviceFn,
    pub huawei_cluster_culling_shader: ash::huawei::cluster_culling_shader::DeviceFn,
    pub huawei_invocation_mask: ash::huawei::invocation_mask::DeviceFn,
    pub huawei_subpass_shading: ash::huawei::subpass_shading::DeviceFn,
    pub intel_performance_query: ash::intel::performance_query::DeviceFn,
    pub nvx_binary_import: ash::nvx::binary_import::DeviceFn,
    pub nvx_image_view_handle: ash::nvx::image_view_handle::DeviceFn,
    pub nv_clip_space_w_scaling: ash::nv::clip_space_w_scaling::DeviceFn,
    pub nv_copy_memory_indirect: ash::nv::copy_memory_indirect::DeviceFn,
    pub nv_cuda_kernel_launch: ash::nv::cuda_kernel_launch::DeviceFn,
    pub nv_device_diagnostic_checkpoints: ash::nv::device_diagnostic_checkpoints::DeviceFn,
    pub nv_device_generated_commands: ash::nv::device_generated_commands::DeviceFn,
    pub nv_device_generated_commands_compute: ash::nv::device_generated_commands_compute::DeviceFn,
    pub nv_external_memory_rdma: ash::nv::external_memory_rdma::DeviceFn,
    pub nv_external_memory_win32: ash::nv::external_memory_win32::DeviceFn,
    pub nv_fragment_shading_rate_enums: ash::nv::fragment_shading_rate_enums::DeviceFn,
    pub nv_low_latency2: ash::nv::low_latency2::DeviceFn,
    pub nv_memory_decompression: ash::nv::memory_decompression::DeviceFn,
    pub nv_mesh_shader: ash::nv::mesh_shader::DeviceFn,
    pub nv_optical_flow: ash::nv::optical_flow::DeviceFn,
    pub nv_ray_tracing: ash::nv::ray_tracing::DeviceFn,
    pub nv_scissor_exclusive: ash::nv::scissor_exclusive::DeviceFn,
    pub nv_shading_rate_image: ash::nv::shading_rate_image::DeviceFn,
    pub qcom_tile_properties: ash::qcom::tile_properties::DeviceFn,
    pub qnx_external_memory_screen_buffer: ash::qnx::external_memory_screen_buffer::DeviceFn,
    pub valve_descriptor_set_host_mapping: ash::valve::descriptor_set_host_mapping::DeviceFn,
    pub _ne: crate::NonExhaustive,
}
impl DeviceFunctions {
    pub(crate) fn load<F>(mut load_fn: F) -> DeviceFunctions
    where
        F: FnMut(&CStr) -> *const c_void,
    {
        DeviceFunctions {
            v1_0: ash::DeviceFnV1_0::load(&mut load_fn),
            v1_1: ash::DeviceFnV1_1::load(&mut load_fn),
            v1_2: ash::DeviceFnV1_2::load(&mut load_fn),
            v1_3: ash::DeviceFnV1_3::load(&mut load_fn),
            khr_acceleration_structure: ash::khr::acceleration_structure::DeviceFn::load(
                &mut load_fn,
            ),
            khr_bind_memory2: ash::khr::bind_memory2::DeviceFn::load(&mut load_fn),
            khr_buffer_device_address: ash::khr::buffer_device_address::DeviceFn::load(
                &mut load_fn,
            ),
            khr_calibrated_timestamps: ash::khr::calibrated_timestamps::DeviceFn::load(
                &mut load_fn,
            ),
            khr_copy_commands2: ash::khr::copy_commands2::DeviceFn::load(&mut load_fn),
            khr_create_renderpass2: ash::khr::create_renderpass2::DeviceFn::load(
                &mut load_fn,
            ),
            khr_deferred_host_operations: ash::khr::deferred_host_operations::DeviceFn::load(
                &mut load_fn,
            ),
            khr_descriptor_update_template: ash::khr::descriptor_update_template::DeviceFn::load(
                &mut load_fn,
            ),
            khr_device_group: ash::khr::device_group::DeviceFn::load(&mut load_fn),
            khr_display_swapchain: ash::khr::display_swapchain::DeviceFn::load(
                &mut load_fn,
            ),
            khr_draw_indirect_count: ash::khr::draw_indirect_count::DeviceFn::load(
                &mut load_fn,
            ),
            khr_dynamic_rendering: ash::khr::dynamic_rendering::DeviceFn::load(
                &mut load_fn,
            ),
            khr_dynamic_rendering_local_read: ash::khr::dynamic_rendering_local_read::DeviceFn::load(
                &mut load_fn,
            ),
            khr_external_fence_fd: ash::khr::external_fence_fd::DeviceFn::load(
                &mut load_fn,
            ),
            khr_external_fence_win32: ash::khr::external_fence_win32::DeviceFn::load(
                &mut load_fn,
            ),
            khr_external_memory_fd: ash::khr::external_memory_fd::DeviceFn::load(
                &mut load_fn,
            ),
            khr_external_memory_win32: ash::khr::external_memory_win32::DeviceFn::load(
                &mut load_fn,
            ),
            khr_external_semaphore_fd: ash::khr::external_semaphore_fd::DeviceFn::load(
                &mut load_fn,
            ),
            khr_external_semaphore_win32: ash::khr::external_semaphore_win32::DeviceFn::load(
                &mut load_fn,
            ),
            khr_fragment_shading_rate: ash::khr::fragment_shading_rate::DeviceFn::load(
                &mut load_fn,
            ),
            khr_get_memory_requirements2: ash::khr::get_memory_requirements2::DeviceFn::load(
                &mut load_fn,
            ),
            khr_line_rasterization: ash::khr::line_rasterization::DeviceFn::load(
                &mut load_fn,
            ),
            khr_maintenance1: ash::khr::maintenance1::DeviceFn::load(&mut load_fn),
            khr_maintenance3: ash::khr::maintenance3::DeviceFn::load(&mut load_fn),
            khr_maintenance4: ash::khr::maintenance4::DeviceFn::load(&mut load_fn),
            khr_maintenance5: ash::khr::maintenance5::DeviceFn::load(&mut load_fn),
            khr_maintenance6: ash::khr::maintenance6::DeviceFn::load(&mut load_fn),
            khr_map_memory2: ash::khr::map_memory2::DeviceFn::load(&mut load_fn),
            khr_performance_query: ash::khr::performance_query::DeviceFn::load(
                &mut load_fn,
            ),
            khr_pipeline_executable_properties: ash::khr::pipeline_executable_properties::DeviceFn::load(
                &mut load_fn,
            ),
            khr_present_wait: ash::khr::present_wait::DeviceFn::load(&mut load_fn),
            khr_push_descriptor: ash::khr::push_descriptor::DeviceFn::load(&mut load_fn),
            khr_ray_tracing_maintenance1: ash::khr::ray_tracing_maintenance1::DeviceFn::load(
                &mut load_fn,
            ),
            khr_ray_tracing_pipeline: ash::khr::ray_tracing_pipeline::DeviceFn::load(
                &mut load_fn,
            ),
            khr_sampler_ycbcr_conversion: ash::khr::sampler_ycbcr_conversion::DeviceFn::load(
                &mut load_fn,
            ),
            khr_shared_presentable_image: ash::khr::shared_presentable_image::DeviceFn::load(
                &mut load_fn,
            ),
            khr_swapchain: ash::khr::swapchain::DeviceFn::load(&mut load_fn),
            khr_synchronization2: ash::khr::synchronization2::DeviceFn::load(
                &mut load_fn,
            ),
            khr_timeline_semaphore: ash::khr::timeline_semaphore::DeviceFn::load(
                &mut load_fn,
            ),
            khr_video_decode_queue: ash::khr::video_decode_queue::DeviceFn::load(
                &mut load_fn,
            ),
            khr_video_encode_queue: ash::khr::video_encode_queue::DeviceFn::load(
                &mut load_fn,
            ),
            khr_video_queue: ash::khr::video_queue::DeviceFn::load(&mut load_fn),
            ext_attachment_feedback_loop_dynamic_state: ash::ext::attachment_feedback_loop_dynamic_state::DeviceFn::load(
                &mut load_fn,
            ),
            ext_buffer_device_address: ash::ext::buffer_device_address::DeviceFn::load(
                &mut load_fn,
            ),
            ext_calibrated_timestamps: ash::ext::calibrated_timestamps::DeviceFn::load(
                &mut load_fn,
            ),
            ext_color_write_enable: ash::ext::color_write_enable::DeviceFn::load(
                &mut load_fn,
            ),
            ext_conditional_rendering: ash::ext::conditional_rendering::DeviceFn::load(
                &mut load_fn,
            ),
            ext_debug_marker: ash::ext::debug_marker::DeviceFn::load(&mut load_fn),
            ext_debug_utils: ash::ext::debug_utils::DeviceFn::load(&mut load_fn),
            ext_depth_bias_control: ash::ext::depth_bias_control::DeviceFn::load(
                &mut load_fn,
            ),
            ext_descriptor_buffer: ash::ext::descriptor_buffer::DeviceFn::load(
                &mut load_fn,
            ),
            ext_device_fault: ash::ext::device_fault::DeviceFn::load(&mut load_fn),
            ext_discard_rectangles: ash::ext::discard_rectangles::DeviceFn::load(
                &mut load_fn,
            ),
            ext_display_control: ash::ext::display_control::DeviceFn::load(&mut load_fn),
            ext_extended_dynamic_state: ash::ext::extended_dynamic_state::DeviceFn::load(
                &mut load_fn,
            ),
            ext_extended_dynamic_state2: ash::ext::extended_dynamic_state2::DeviceFn::load(
                &mut load_fn,
            ),
            ext_extended_dynamic_state3: ash::ext::extended_dynamic_state3::DeviceFn::load(
                &mut load_fn,
            ),
            ext_external_memory_host: ash::ext::external_memory_host::DeviceFn::load(
                &mut load_fn,
            ),
            ext_full_screen_exclusive: ash::ext::full_screen_exclusive::DeviceFn::load(
                &mut load_fn,
            ),
            ext_hdr_metadata: ash::ext::hdr_metadata::DeviceFn::load(&mut load_fn),
            ext_host_image_copy: ash::ext::host_image_copy::DeviceFn::load(&mut load_fn),
            ext_host_query_reset: ash::ext::host_query_reset::DeviceFn::load(
                &mut load_fn,
            ),
            ext_image_compression_control: ash::ext::image_compression_control::DeviceFn::load(
                &mut load_fn,
            ),
            ext_image_drm_format_modifier: ash::ext::image_drm_format_modifier::DeviceFn::load(
                &mut load_fn,
            ),
            ext_line_rasterization: ash::ext::line_rasterization::DeviceFn::load(
                &mut load_fn,
            ),
            ext_mesh_shader: ash::ext::mesh_shader::DeviceFn::load(&mut load_fn),
            ext_metal_objects: ash::ext::metal_objects::DeviceFn::load(&mut load_fn),
            ext_multi_draw: ash::ext::multi_draw::DeviceFn::load(&mut load_fn),
            ext_opacity_micromap: ash::ext::opacity_micromap::DeviceFn::load(
                &mut load_fn,
            ),
            ext_pageable_device_local_memory: ash::ext::pageable_device_local_memory::DeviceFn::load(
                &mut load_fn,
            ),
            ext_pipeline_properties: ash::ext::pipeline_properties::DeviceFn::load(
                &mut load_fn,
            ),
            ext_private_data: ash::ext::private_data::DeviceFn::load(&mut load_fn),
            ext_sample_locations: ash::ext::sample_locations::DeviceFn::load(
                &mut load_fn,
            ),
            ext_shader_module_identifier: ash::ext::shader_module_identifier::DeviceFn::load(
                &mut load_fn,
            ),
            ext_shader_object: ash::ext::shader_object::DeviceFn::load(&mut load_fn),
            ext_swapchain_maintenance1: ash::ext::swapchain_maintenance1::DeviceFn::load(
                &mut load_fn,
            ),
            ext_transform_feedback: ash::ext::transform_feedback::DeviceFn::load(
                &mut load_fn,
            ),
            ext_validation_cache: ash::ext::validation_cache::DeviceFn::load(
                &mut load_fn,
            ),
            ext_vertex_input_dynamic_state: ash::ext::vertex_input_dynamic_state::DeviceFn::load(
                &mut load_fn,
            ),
            amdx_shader_enqueue: ash::amdx::shader_enqueue::DeviceFn::load(&mut load_fn),
            amd_buffer_marker: ash::amd::buffer_marker::DeviceFn::load(&mut load_fn),
            amd_display_native_hdr: ash::amd::display_native_hdr::DeviceFn::load(
                &mut load_fn,
            ),
            amd_draw_indirect_count: ash::amd::draw_indirect_count::DeviceFn::load(
                &mut load_fn,
            ),
            amd_shader_info: ash::amd::shader_info::DeviceFn::load(&mut load_fn),
            android_external_memory_android_hardware_buffer: ash::android::external_memory_android_hardware_buffer::DeviceFn::load(
                &mut load_fn,
            ),
            fuchsia_buffer_collection: ash::fuchsia::buffer_collection::DeviceFn::load(
                &mut load_fn,
            ),
            fuchsia_external_memory: ash::fuchsia::external_memory::DeviceFn::load(
                &mut load_fn,
            ),
            fuchsia_external_semaphore: ash::fuchsia::external_semaphore::DeviceFn::load(
                &mut load_fn,
            ),
            google_display_timing: ash::google::display_timing::DeviceFn::load(
                &mut load_fn,
            ),
            huawei_cluster_culling_shader: ash::huawei::cluster_culling_shader::DeviceFn::load(
                &mut load_fn,
            ),
            huawei_invocation_mask: ash::huawei::invocation_mask::DeviceFn::load(
                &mut load_fn,
            ),
            huawei_subpass_shading: ash::huawei::subpass_shading::DeviceFn::load(
                &mut load_fn,
            ),
            intel_performance_query: ash::intel::performance_query::DeviceFn::load(
                &mut load_fn,
            ),
            nvx_binary_import: ash::nvx::binary_import::DeviceFn::load(&mut load_fn),
            nvx_image_view_handle: ash::nvx::image_view_handle::DeviceFn::load(
                &mut load_fn,
            ),
            nv_clip_space_w_scaling: ash::nv::clip_space_w_scaling::DeviceFn::load(
                &mut load_fn,
            ),
            nv_copy_memory_indirect: ash::nv::copy_memory_indirect::DeviceFn::load(
                &mut load_fn,
            ),
            nv_cuda_kernel_launch: ash::nv::cuda_kernel_launch::DeviceFn::load(
                &mut load_fn,
            ),
            nv_device_diagnostic_checkpoints: ash::nv::device_diagnostic_checkpoints::DeviceFn::load(
                &mut load_fn,
            ),
            nv_device_generated_commands: ash::nv::device_generated_commands::DeviceFn::load(
                &mut load_fn,
            ),
            nv_device_generated_commands_compute: ash::nv::device_generated_commands_compute::DeviceFn::load(
                &mut load_fn,
            ),
            nv_external_memory_rdma: ash::nv::external_memory_rdma::DeviceFn::load(
                &mut load_fn,
            ),
            nv_external_memory_win32: ash::nv::external_memory_win32::DeviceFn::load(
                &mut load_fn,
            ),
            nv_fragment_shading_rate_enums: ash::nv::fragment_shading_rate_enums::DeviceFn::load(
                &mut load_fn,
            ),
            nv_low_latency2: ash::nv::low_latency2::DeviceFn::load(&mut load_fn),
            nv_memory_decompression: ash::nv::memory_decompression::DeviceFn::load(
                &mut load_fn,
            ),
            nv_mesh_shader: ash::nv::mesh_shader::DeviceFn::load(&mut load_fn),
            nv_optical_flow: ash::nv::optical_flow::DeviceFn::load(&mut load_fn),
            nv_ray_tracing: ash::nv::ray_tracing::DeviceFn::load(&mut load_fn),
            nv_scissor_exclusive: ash::nv::scissor_exclusive::DeviceFn::load(
                &mut load_fn,
            ),
            nv_shading_rate_image: ash::nv::shading_rate_image::DeviceFn::load(
                &mut load_fn,
            ),
            qcom_tile_properties: ash::qcom::tile_properties::DeviceFn::load(
                &mut load_fn,
            ),
            qnx_external_memory_screen_buffer: ash::qnx::external_memory_screen_buffer::DeviceFn::load(
                &mut load_fn,
            ),
            valve_descriptor_set_host_mapping: ash::valve::descriptor_set_host_mapping::DeviceFn::load(
                &mut load_fn,
            ),
            _ne: crate::NonExhaustive(()),
        }
    }
}
impl std::fmt::Debug for DeviceFunctions {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_struct(stringify!(DeviceFunctions)).finish_non_exhaustive()
    }
}
