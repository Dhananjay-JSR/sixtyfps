/* LICENSE BEGIN
    This file is part of the SixtyFPS Project -- https://sixtyfps.io
    Copyright (c) 2021 Olivier Goffart <olivier.goffart@sixtyfps.io>
    Copyright (c) 2021 Simon Hausmann <simon.hausmann@sixtyfps.io>

    SPDX-License-Identifier: GPL-3.0-only
    This file is also available under commercial licensing terms.
    Please contact info@sixtyfps.io for more information.
LICENSE END */

mod apply_default_properties_from_style;
mod binding_analysis;
mod check_expressions;
mod check_public_api;
mod clip;
mod collect_custom_fonts;
mod collect_globals;
mod collect_structs;
mod compile_paths;
mod deduplicate_property_read;
mod default_geometry;
mod embed_resources;
mod ensure_window;
mod flickable;
mod focus_item;
mod generate_item_indices;
mod infer_aliases_types;
mod inlining;
mod lower_layout;
mod lower_popups;
mod lower_shadows;
mod lower_states;
mod materialize_fake_properties;
mod move_declarations;
mod remove_aliases;
mod remove_unused_properties;
mod repeater_component;
mod resolve_native_classes;
mod resolving;
mod transform_and_opacity;
mod unique_id;
mod z_order;

use super::*;

pub async fn run_passes(
    doc: &object_tree::Document,
    diag: &mut diagnostics::BuildDiagnostics,
    mut type_loader: &mut typeloader::TypeLoader<'_>,
    compiler_config: &CompilerConfiguration,
) {
    let global_type_registry = type_loader.global_type_registry.clone();
    passes::infer_aliases_types::resolve_aliases(doc, diag);
    passes::resolving::resolve_expressions(doc, type_loader, diag);
    passes::check_expressions::check_expressions(doc, diag);
    passes::check_public_api::check_public_api(&doc.root_component, diag);
    passes::inlining::inline(doc);
    passes::compile_paths::compile_paths(&doc.root_component, &doc.local_registry, diag);
    passes::unique_id::assign_unique_id(&doc.root_component);
    passes::focus_item::resolve_element_reference_in_set_focus_calls(&doc.root_component, diag);
    passes::focus_item::determine_initial_focus_item(&doc.root_component, diag);
    passes::focus_item::erase_forward_focus_properties(&doc.root_component);
    passes::flickable::handle_flickable(&doc.root_component, &global_type_registry.borrow());
    if compiler_config.embed_resources {
        passes::embed_resources::embed_resources(&doc.root_component);
    }
    passes::lower_states::lower_states(&doc.root_component, &doc.local_registry, diag);
    passes::repeater_component::process_repeater_components(&doc.root_component);
    passes::lower_popups::lower_popups(&doc.root_component, &doc.local_registry, diag);
    passes::lower_layout::lower_layouts(&doc.root_component, &global_type_registry.borrow(), diag);
    passes::z_order::reorder_by_z_order(&doc.root_component, diag);
    passes::lower_shadows::lower_shadow_properties(&doc.root_component, &doc.local_registry, diag);
    passes::clip::handle_clip(&doc.root_component, &global_type_registry.borrow(), diag);
    passes::transform_and_opacity::handle_transform_and_opacity(
        &doc.root_component,
        &global_type_registry.borrow(),
        diag,
    );
    passes::default_geometry::default_geometry(&doc.root_component, diag);
    passes::materialize_fake_properties::materialize_fake_properties(&doc.root_component);
    passes::apply_default_properties_from_style::apply_default_properties_from_style(
        &doc.root_component,
        &mut type_loader,
        diag,
    )
    .await;
    passes::ensure_window::ensure_window(&doc.root_component, &doc.local_registry);
    passes::collect_globals::collect_globals(&doc.root_component, diag);
    passes::binding_analysis::binding_analysis(&doc.root_component, diag);
    passes::deduplicate_property_read::deduplicate_property_read(&doc.root_component);
    passes::move_declarations::move_declarations(&doc.root_component, diag);
    passes::remove_aliases::remove_aliases(&doc.root_component, diag);
    passes::resolve_native_classes::resolve_native_classes(&doc.root_component);
    passes::remove_unused_properties::remove_unused_properties(&doc.root_component);
    passes::collect_structs::collect_structs(&doc.root_component, diag);
    passes::generate_item_indices::generate_item_indices(&doc.root_component);
    passes::collect_custom_fonts::collect_custom_fonts(
        &doc.root_component,
        std::iter::once(&*doc).chain(type_loader.all_documents()),
        compiler_config.embed_resources,
    );
}

/// Run the passes on imported documents
pub fn run_import_passes(
    doc: &object_tree::Document,
    type_loader: &typeloader::TypeLoader<'_>,
    diag: &mut diagnostics::BuildDiagnostics,
) {
    crate::passes::infer_aliases_types::resolve_aliases(doc, diag);
    crate::passes::resolving::resolve_expressions(doc, type_loader, diag);
    crate::passes::check_expressions::check_expressions(doc, diag);
}