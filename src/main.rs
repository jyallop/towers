extern crate nannou;
extern crate rand;
use nannou::prelude::*;
use rand::prelude::*;

const ROTATION_SCALE : f32 = 10.0;
//const ROTATION_SCALE : f32 = 20.5;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

struct Tower {
    x: f32,
    base: Vec<Point3>,
    arms: Vec<Vec<Point3>>,
    rotation_speed: f32,
    color: Rgb<u8>,
}

struct Model {
    towers: Vec<Tower>,
    color_choices: Vec<Rgb<u8>>,
    background: Rgb<u8>,
    pause_end: Option<u64>,
    next_tower: Vec<usize>

}

fn model(app: &App) -> Model {
    let mut rng = rand::rng();
    let dims = app.window_rect(); // => Rect
    let color_choices = vec![
	//Red
	(Rgb::new(212, 245, 214), Rgb::new(212, 106, 106),
	 Rgb::new(128, 21, 21), Rgb::new(85, 0, 0)),
	//Blue
    	(Rgb::new(255, 241, 221), Rgb::new(77, 101, 141),
	 Rgb::new(21, 44, 85), Rgb::new(5, 24, 56)),
	//Green
	(Rgb::new(255, 231, 221), Rgb::new(0, 58, 33),
	 Rgb::new(15, 87, 56), Rgb::new(73, 146, 115))
    ];
    let colors = color_choices.choose(&mut rng).unwrap();

    let arm_scale = 0.1;
    let arm_width_scale = 0.75;

    let large_width = dims.w() * 1.0;
    let large_x = dims.left() + dims.w() * 0.2;
    let large_y = dims.bottom();
    let large_height = dims.h() * 2.0;
    let large_arm_starts = vec![
	Point3::new(large_x + large_width / 2.0, large_y + large_height / 2.0 - arm_scale * large_height, 0.0),
	Point3::new(large_x + large_width / 2.0, large_y + large_height / 2.0 - arm_scale * large_height, PI),
	Point3::new(large_x + large_width / 2.0, large_y + large_height / 2.0 - 2.0 * arm_scale * large_height, PI / 2.0),
	Point3::new(large_x + large_width / 2.0, large_y + large_height / 2.0 - 2.0 * arm_scale * large_height, 3.0 * PI / 2.0)
    ];
    let large_rotation_offset = 1.0;

    let tower_one = Tower {
	x: large_x,
	base: rect_points(large_x, large_y, large_width, large_height),
	arms: large_arm_starts.into_iter()
	    .map(|arm|
		 rotate_points_xz(large_x, 0.0, arm.z + large_rotation_offset,
				  arm_points(arm.x, arm.y, large_height * arm_scale, large_width * arm_width_scale,
					     large_width * arm_width_scale * 0.1))).collect(),
	color: colors.1,
	rotation_speed: ROTATION_SCALE / 1000.0,
    };

    let medium_width = dims.w() * 0.45;
    let medium_x = dims.right() - medium_width / 2.0;
    let medium_height = dims.h() * 1.6;
    let medium_y = dims.bottom() - medium_height * 1.0 / 12.0;

    let medium_arm_starts = vec![
	Point3::new(medium_x + medium_width / 2.0, medium_y + medium_height / 2.0 - arm_scale * medium_height, 0.0),
	Point3::new(medium_x + medium_width / 2.0, medium_y + medium_height / 2.0 - arm_scale * medium_height, PI),

	Point3::new(medium_x + medium_width / 2.0, medium_y, PI),
	Point3::new(medium_x + medium_width / 2.0, medium_y, 0.0),
	
	Point3::new(medium_x + medium_width / 2.0, medium_y - arm_scale * medium_height, PI),
	Point3::new(medium_x + medium_width / 2.0, medium_y - arm_scale * medium_height, 0.0)
    ];

    let medium_rotation_offset = 0.1;

    let tower_two = Tower {
	x: medium_x,
	base: rect_points(medium_x, medium_y, medium_width, medium_height),
	arms: medium_arm_starts.into_iter()
	    .map(|arm| rotate_points_xz(medium_x, 0.0, arm.z + medium_rotation_offset,
					arm_points(arm.x, arm.y, medium_height * arm_scale,
						   medium_width * arm_width_scale,
						   medium_width * arm_width_scale * 0.1))).collect(),
	color: colors.2,
	rotation_speed: ROTATION_SCALE / 500.0,
    };

    let small_width = dims.w() * 0.3;
    let small_x = dims.left() + small_width * 0.55;
    let small_height = dims.h() * 1.2;
    let small_y = dims.bottom() - small_height * 0.1;
    let small_arm_starts = vec![
	Point3::new(small_x + small_width / 2.0, small_y + small_height / 2.0 - arm_scale * small_height, 0.0),
	Point3::new(small_x + small_width / 2.0, small_y + small_height / 2.0 - arm_scale * small_height, PI),

	Point3::new(small_x + small_width / 2.0,
		    small_y + small_height / 2.75 - arm_scale * small_height, 2.0 * PI / 3.0),
	Point3::new(small_x + small_width / 2.0,
		    small_y + small_height / 2.75 - arm_scale * small_height, 5.0 * PI / 3.0),
	
	Point3::new(small_x + small_width / 2.0,
		    small_y + small_height / 4.5 - arm_scale * small_height, PI / 3.0),
	Point3::new(small_x + small_width / 2.0,
		    small_y + small_height / 4.5 - arm_scale * small_height, 4.0 * PI / 3.0)
    ];
    let small_rotation_offset = 2.0;

    let tower_three = Tower {
	x: small_x,
	base: rect_points(small_x, small_y, small_width, small_height),
	arms: small_arm_starts.into_iter()
	    .map(|arm| rotate_points_xz(small_x, 0.0, arm.z + small_rotation_offset,
					arm_points(arm.x, arm.y, small_height * arm_scale,
						   small_width * arm_width_scale,
						   small_width * arm_width_scale * 0.1))).collect(),
	color: colors.3,
	rotation_speed: ROTATION_SCALE / 100.0,
    };

    Model {
	towers: vec![tower_one, tower_two, tower_three],
	color_choices: Vec::new(),
	background: colors.0,
	pause_end: Some(180),
	next_tower: Vec::new()
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let mut rng = rand::rng();
    if model.pause_end.unwrap_or(u64::MAX) < app.elapsed_frames() {
	match model.color_choices.pop() {
	    Some(color) => {
		match model.next_tower.pop() {
		    Some(tower_index) => {
			model.towers[tower_index].color = color;
		    }
		    None => {
			model.background = color;
			let mut index = (0..3).collect::<Vec<_>>();
			index.shuffle(&mut rng);
			model.next_tower = index;
		    }
		}
		model.pause_end = Some(app.elapsed_frames() + 30);
	    }
	    None => {
		let color_choices = vec![
		    //Red
		    (Rgb::new(212, 245, 214), Rgb::new(212, 106, 106),
		     Rgb::new(128, 21, 21), Rgb::new(85, 0, 0)),
		    //Blue
    		    (Rgb::new(255, 241, 221), Rgb::new(77, 101, 141),
		     Rgb::new(21, 44, 85), Rgb::new(5, 24, 56)),
		    //Green
		    (Rgb::new(255, 231, 221), Rgb::new(0, 58, 33),
		     Rgb::new(15, 87, 56), Rgb::new(73, 146, 115))
		];
		let next_colors_tuple = color_choices.into_iter()
		    .filter(|p| p.0 != model.background)
		    .choose(&mut rng).unwrap();
		let mut next_colors = vec![next_colors_tuple.1, next_colors_tuple.2, next_colors_tuple.3];
		next_colors.shuffle(&mut rng);
		next_colors.push(next_colors_tuple.0);
		model.color_choices = next_colors;
		model.pause_end = Some(app.elapsed_frames() + 360);
	    }
	}
    }
    for tower in &mut model.towers {
	tower.arms = tower.arms.clone().into_iter()
	    .map(|arm| rotate_points_xz(tower.x, 0.0, tower.rotation_speed, arm)).collect();
    }	     
}

fn rotate_points_xz(x: f32, z: f32, angle: f32, points: Vec<Point3>) -> Vec<Point3> {
    points.into_iter().map(|point| Point3::new(x + (point.x - x) * angle.cos() - (point.z - z)* angle.sin(),
					       point.y,
					       z + (point.x - x) * angle.sin() + (point.z - z) * angle.cos()))
	.collect()
}

fn rect_points(x: f32, y: f32, w: f32, h: f32) -> Vec<Point3> {
    vec![
	Point3::new(x - w / 2.0, y - h / 2.0, 0.0),
	Point3::new(x + w / 2.0, y - h / 2.0, 0.0),
	Point3::new(x + w / 2.0, y + h / 2.0, 0.0),
	Point3::new(x - w / 2.0, y + h / 2.0, 0.0)
    ]
}

fn arm_points(x: f32, y: f32, height: f32, width: f32, start: f32) -> Vec<Point3> {
    let fun_bot = | x : f32 | (x.pow(0.5)) * ((height / 2.0) - height * 0.05) + y;
    let fun_top = | x : f32 | -((x.pow(0.5)) * ((height / 2.0) - height * 0.05)) + y + height;
    let mut output = Vec::new();
    output.push(Point3::new(x, y, 0.0));
    output.push(Point3::new(x, y + height, 0.0));
    output.push(Point3::new(x + start, y + height, 0.0));
    for i in 0..10 {
	let index = (i as f32) / 10.0;
	output.push(Point3::new(x + start + (width - start) * index, fun_top(index), 0.0));
    }
    for i in 1..11 {
	let index = (10.0 - i as f32) / 10.0;
	output.push(Point3::new(x + start + (width - start) * index, fun_bot(index), 0.0));
    }
    output.push(Point3::new(x + start, y, 0.0));
    output
}

fn draw_color_changing(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    frame.clear(model.background);

    let stroke_width = 1.0;
    let stroke_color = BLACK;
    for tower in &model.towers {
	let mut all_points = Vec::new();
	all_points.push(tower.base.clone());
	for arm in &tower.arms {
	    all_points.push((&arm).to_vec());
	}
	all_points.sort_by_key(|a| a.into_iter().map(|p| p.z.round() as i32).min());
	for points in all_points {
	    draw.polygon()
		.stroke(stroke_color)
		.stroke_weight(stroke_width)
		.color(tower.color)
     		.points(points);
	}
    }

    draw.to_frame(app, &frame).unwrap();
}

fn view(app: &App, model: &Model, frame: Frame) {
    draw_color_changing(app, model, frame)
}

