project('puzzled', 'rust',
          version: '1.2.2',
    meson_version: '>= 1.0.0',
  default_options: [ 'warning_level=2', 'werror=false', ],
)

i18n = import('i18n')
gnome = import('gnome')




subdir('data')
subdir('puzzled/resources')
subdir('puzzled/src')

cargo_bin  = find_program('cargo')
cargo_manifest_path_opt  = [ '--manifest-path', meson.project_source_root() / 'Cargo.toml' ]
cargo_target_dir_opt = [ '--target-dir', meson.project_build_root() / 'puzzled' / 'src' ]
cargo_env  = [ 'CARGO_HOME=' + meson.project_build_root() / 'puzzled' / 'cargo-home' ]

test('cargo-test',
  cargo_bin,
  args: [ 'test' ] + cargo_manifest_path_opt + cargo_target_dir_opt,
  env: cargo_env,
  timeout: 600000,
)

test('cargo-fmt-check',
  cargo_bin,
  args: [ 'fmt' ] + cargo_manifest_path_opt + [ '--all', '--check', '--verbose' ],
  env: cargo_env,
  timeout: 60000,
)

test('cargo-clippy',
  cargo_bin,
  args: [ 'clippy', '--all-targets', '--all-features' ] + cargo_manifest_path_opt +[ '--', '-D', 'warnings' ],
  env: cargo_env,
  timeout: 60000,
)

subdir('po')

gnome.post_install(
     glib_compile_schemas: true,
    gtk_update_icon_cache: true,
  update_desktop_database: true,
)
