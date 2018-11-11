import vim
from omnipytent import *
from omnipytent.ext.idan import *


def rust_log_params():
    import toml
    return ','.join(toml.load('Cargo.toml')['workspace']['members'])


@task
def compile(ctx):
    cargo['build', '-q'] & ERUN.bang


@task
def run(ctx):
    cargo['run', '-q'].with_env(RUST_LOG=rust_log_params()) & BANG
