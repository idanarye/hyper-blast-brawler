import vim
from omnipytent import *
from omnipytent.ext.idan import *


def rust_log_params():
    def gen():
        import toml
        yield 'warn'
        for member in toml.load('Cargo.toml')['workspace']['members']:
            yield '%s=debug' % member
    return ','.join(gen())

@task(alias=':0')
def print_log_params(ctx):
    print(rust_log_params())


@task
def compile(ctx):
    cargo['build', '-q'] & ERUN.bang


@task
def run(ctx):
    cargo['run', '-q'].with_env(RUST_LOG=rust_log_params(), RUST_BACKTRACE='1') & BANG
