project('osu_format', 'cpp')

subdir('src')

executable('demo', sources, include_directories : include_dirs, link_args : ['-L' + join_paths(meson.current_source_dir(), 'target/debug'), '-losu', '-lpthread', '-ldl'])
