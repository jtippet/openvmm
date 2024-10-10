// Copyright (C) Microsoft Corporation. All rights reserved.

//! Modules related to the mapping manager.

mod manager;
mod mappable;
mod object_cache;
mod va_mapper;

pub use manager::MappingManager;
pub use manager::MappingManagerClient;
pub use mappable::Mappable;
pub use va_mapper::VaMapper;
pub use va_mapper::VaMapperError;