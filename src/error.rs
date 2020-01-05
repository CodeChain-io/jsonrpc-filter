// Copyright 2019-2020 Kodebox, Inc.
// This file is part of CodeChain.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use hyper::http::Error as HyperHttpError;
use hyper::Error as HyperError;
use serde_json::Error as SerdeError;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug)]
pub enum Error {
    Hyper(HyperError),
    HyperHttp(HyperHttpError),
    Serde(SerdeError),
    NotAllowedMethod(String),
    MethodIsNotString,
    MethodIsNotDefined,
}

impl StdError for Error {}

impl From<SerdeError> for Error {
    fn from(err: SerdeError) -> Self {
        Error::Serde(err)
    }
}

impl From<HyperError> for Error {
    fn from(err: HyperError) -> Self {
        Error::Hyper(err)
    }
}

impl From<HyperHttpError> for Error {
    fn from(err: HyperHttpError) -> Self {
        Error::HyperHttp(err)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Error::Hyper(err) => write!(f, "{}", err),
            Error::HyperHttp(err) => write!(f, "{}", err),
            Error::Serde(err) => write!(f, "{}", err),
            Error::NotAllowedMethod(method) => write!(f, "{} is not allowed method", method),
            Error::MethodIsNotString => write!(f, "Method is not a string"),
            Error::MethodIsNotDefined => write!(f, "Method is not defined"),
        }
    }
}
