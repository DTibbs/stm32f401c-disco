# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

### Changed

- Since this is starting from a fork of the f3 board support crate, use stm32f4 crate dependencies.

- LED count down to only the 4 user LEDs for f4 discovery board.

### Removed

- Dependencies on accel/gyro that don't exist on f4 discovery board.

### Added

- TODO Coments where the drivers for accel and DAC will go for f4 discovery board.