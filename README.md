# UI Screens for BevyEngine

![version](https://img.shields.io/crates/v/gerg_ui)
![downloads](https://img.shields.io/crates/d/gerg_ui)

This is a little thing I put together for creating simple UI screens using the BevyEngine.
The idea is to define the screens in a sort of poor-man's markup and this crate will then
provide some functions to create the UI 'objects'.

For example, the following in a file will create a screen looking like:

![sample_picture](https://raw.githubusercontent.com/gmoller/gerg-ui/main/Capture.PNG)

with the calling code:
```sh
let lines = gerg_ui::read_ui_file("screen1.ui");
let controls = gerg_ui::instantiate_controls(lines);
gerg_ui::spawn_controls(&mut commands, asset_server, materials, controls), Vec2::new(1920.0, 1080.0);
```

```sh
--global_settings--
font_name: CrimsonText-Regular.ttf
font_size: 30
color: 1;1;1 // WHITE
--end--

--picture_box--
name: frame1
texture_name: big_frame.png
size: 300;450
center_position: -800;300
draw_order: 0
--end--

--picture_box--
name: frame2
texture_name: small_frame.png
size: 200;300
draw_order: 0.1
dock_with: frame1.top_middle<->this.top_middle
offset: 0;-30
--end--

--picture_box--
name: frame3
texture_name: big_frame.png
size: 300;450
draw_order: 0.2
dock_with: frame1.center_right<->this.center_left
offset: 50;0
--end--

--label--
name: label1
size: 100;50
text_string: Test1
font_size: 50
color: 0;1;1 // CYAN
dock_with: frame2.top_left<->this.top_left
offset: 15;-15
--end--

--label--
name: label2
size: 100;50
text_string: Test2
font_name: CrimsonText-Bold.ttf
font_size: 50
color: 1;0.012;0.243 // AMERICAN ROSE
dock_with: label1.bottom_left<->this.top_left
--end--

--label--
name: label3
size: 270;420
text_string: How great was the West Indian cricket team of the 70''s and 80''s? Marshall, Holding, Garner, Croft, Roberts bowling... Richards, Greenidge, Haynes batting. Brilliant stuff. Were the Aussies of the 90''s, 2000''s better, I don''t think so, but Warne and McGrath made a formidable combination.
font_size: 25
color: 1;0.937;0 // CANARY YELLOW
dock_with: frame3.top_left<->this.top_left
offset: 15;-15
--end--
```
