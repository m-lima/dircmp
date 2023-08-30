#!/usr/bin/env bash
export SLINT_STYLE=fluent
export SLINT_BACKEND=winit

cargo ${@}
