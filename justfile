ranim scene:
    cargo run -p {{scene}} --release

manim scene:
    uv run manimgl -w "./packages/{{scene}}/manim.py" --resolution 1920x1080 --fps 60 --color "#333333" --video_dir ./output/{{scene}}

manimce scene:
    uv run manim "./packages/{{scene}}/manimce.py" --resolution 1920,1080 --fps 60 --media_dir ./output/{{scene}}

janim scene:
    uv run janim write "./packages/{{scene}}/janim.py" -c background_color "#333333" -c fps 60 -c frame_height 8 -c pixel_width 1920 -c pixel_height 1080 -c output_dir ./output/{{scene}}

gjanim scene:
    janim write "./packages/{{scene}}/janim.py" -c background_color "#333333" -c fps 60 -c frame_height 8 -c pixel_width 1920 -c pixel_height 1080 -c output_dir ./output/{{scene}}