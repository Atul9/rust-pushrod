// Pushrod
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

/// Main module containing the run loop for the UI components, containers for windows and
/// `PushrodWidget` trait objects, and so on.  Contains the core elements required to build
/// a UI.
pub mod core;

/// Companion module used to define and trigger system-wide events.  Uses an event masking
/// style similar to the Atari ST GEM series: event masks can be used to tell the Pushrod
/// run loop which events the programmer desires to receive.
pub mod event;

/// Widget library used for on-screen UI interaction.  This is a core set of `PushrodWidget`
/// objects that are used to allow users to interact with an application.  Contains a core set
/// of widgets that can be extended.
pub mod widget;
