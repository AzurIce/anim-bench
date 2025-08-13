clean-ranim scene:
    -rm -rf ./output/{{scene}}/Ranim*.mp4
clean-janim scene:
    -rm -rf ./output/{{scene}}/Janim.mp4
clean-manim scene:
    -rm -rf ./output/{{scene}}/Manim.mp4
clean-manimce scene:
    -rm -rf ./output/{{scene}}/videos/
    -rm -rf ./output/{{scene}}/images/

ranim scene:
    clean-ranim {{scene}}
    cargo run -p {{scene}} --release

manim scene:
    clean-manim {{scene}}
    uv run manimgl -w "./packages/{{scene}}/manim.py" --resolution 1920x1080 --fps 60 --color "#333333" --video_dir ./output/{{scene}}

manimce scene:
    clean-manimce {{scene}}
    uv run manim "./packages/{{scene}}/manimce.py" --resolution 1920,1080 --fps 60 --media_dir ./output/{{scene}}

janim scene:
    clean-janim {{scene}}
    uv run janim write "./packages/{{scene}}/janim.py" -c background_color "#333333" -c fps 60 -c frame_height 8 -c pixel_width 1920 -c pixel_height 1080 -c output_dir ./output/{{scene}}

gjanim scene:
    janim write "./packages/{{scene}}/janim.py" -c background_color "#333333" -c fps 60 -c frame_height 8 -c pixel_width 1920 -c pixel_height 1080 -c output_dir ./output/{{scene}}