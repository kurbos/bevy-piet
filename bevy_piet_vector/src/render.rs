use std::cmp::Ordering;

use bevy::prelude::*;
use bevy::math::Vec3Swizzles;
use bevy_piet_render::RenderWorld;
use kurbo::{Affine, BezPath, Circle, Line, Point, Rect, Shape};

use piet_gpu::{PietGpuRenderContext, PicoSvg, RenderContext};

use crate::vector_image::{ExtractedVecImgInstances, VectorImageRenderAssets};

pub fn prepare_vector_images(
    mut extracted_app_world_vecs: ResMut<ExtractedVecImgInstances>,
    vec_images: Res<VectorImageRenderAssets>,
    mut ctx: ResMut<PietGpuRenderContext>,
) {
    // Sort sprites by z for correct transparency and then by handle to improve batching
    extracted_app_world_vecs.instances.sort_unstable_by(|a, b| {
        match a
            .transform
            .translation
            .z
            .partial_cmp(&b.transform.translation.z)
        {
            Some(Ordering::Equal) | None => a.vec_image_handle_id.cmp(&b.vec_image_handle_id),
            Some(other) => other,
        }
    });

    for extracted_inst in extracted_app_world_vecs.instances.iter() {

        if let Some(vec_image) =
            vec_images.get(&Handle::weak(extracted_inst.vec_image_handle_id))
        {
            render_svg(&vec_image.svg, &mut ctx, extracted_inst.transform);
        }
    }
}

pub fn render_svg(svg: &PicoSvg, rc: &mut PietGpuRenderContext, transform: GlobalTransform) {
    let trans = kurbo::Vec2::new(transform.translation.x as f64, transform.translation.y as f64);
    rc.transform(Affine::translate(trans) * Affine::rotate(0.01 as f64));
    svg.render(rc);
}
