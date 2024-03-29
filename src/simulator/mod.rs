mod balloon;
mod bus;
pub mod config;
mod constants;
mod forces;
mod gas;
mod heat;
pub mod io;
pub mod schedule;

pub trait SolidBody {
    fn drag_area(&self) -> f32;
    fn drag_coeff(&self) -> f32;
    fn total_mass(&self) -> f32;
}
