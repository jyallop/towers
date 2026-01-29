fn min(x: f32, y: f32) -> f32 {
    if x < y {
	x
    } else {
	y
    }
}

struct Tower {
    width: f32,
    height: f32,
    x_pos: f32,
    arms: i32,
    color: Rgb<u8>
}

struct Model {
    tower_left: Tower,
    tower_right: Tower,
    background_color: Rgb<u8>,
}

fn rotating_model(app: &App) -> Model {
    let color_schemes = vec![
	(GREEN, DARKSLATEGREY, DARKVIOLET),
	(PAPAYAWHIP, DARKRED, CRIMSON)
    ];

    let frame = app.window_rect(); // => Rect
    let mut rng = rand::rng();
    let height_one = rng.random_range(frame.h() * 0.6..frame.h() * 0.8);
    let tower_width_l = rng.random_range(frame.w() * 0.1..frame.w() * 0.3);
    let tower_height_l = height_one;
    let tower_x_l = rng.random_range(-(frame.w() * 0.2)..(frame.w() * 0.2));
    let tower_arms_l = rng.random_range(3..6);

    let x_two_range = if tower_x_l > 0.0 {
	-(frame.w() * 0.2)..0.0
    } else {
	0.0..(frame.w() * 0.2)
    };

    let tower_width_r = rng.random_range(frame.w() * 0.1..frame.w() * 0.3);
    let tower_height_r = rng.random_range(frame.h() * 0.2..frame.h() * 0.4);
    let tower_x_r = rng.random_range(x_two_range);
    let tower_arms_r = rng.random_range(3..6);

    let scheme = color_schemes.choose(&mut rng).unwrap();
    Model {
	tower_left: Tower {
	    width : tower_width_l,
	    height: tower_height_l,
	    x_pos: tower_x_l,
	    arms: tower_arms_l,
	    color: scheme.1
	},
	tower_right: Tower {
	    width : tower_width_r,
	    height: tower_height_r,
	    x_pos: tower_x_r,
	    arms: tower_arms_r,
	    color: scheme.2
	},
	background_color: scheme.0
    }
}

fn model(app: &App) -> Model {
    if CHANGING {
	rotating_model(app)
    } else {
	rotating_model(app)
    }
}
fn update(app: &App, model: &mut Model, _update: Update) {
    if CHANGING {
	if app.elapsed_frames() % 600 == 0 {
	    *model = rotating_model(app);
	}
    } else {
	if app.elapsed_frames() % 600 == 0 {
	    *model = rotating_model(app);
	}
    }
}
fn generate_polygon_points(x: f32, y: f32, radius: f32, sides: i32, angle: f32) -> Vec<Point2> {
    let alpha = 2.0 * PI / (sides as f32);
    (1..sides + 1).scan(Point2::new(x - radius * angle.sin(), y + radius * angle.cos()),
		    |prev, _i| {
			*prev = Point2::new(x +
					    (((*prev).x - x) * alpha.cos()) -
					    (((*prev).y - y) * alpha.sin()),
					    y +
					    (((*prev).x - x) * alpha.sin()) +
					    (((*prev).y - y) * alpha.cos()));
			Some(*prev)
		    }).collect()
}

fn grid(x: f32, y: f32, cols: i32, rows: i32,  width: f32, offset: f32, radius: f32, angle: f32) -> Vec<Point2> {
    let width_x = width * (angle - (PI / 2.0)).cos();
    let width_y = width * (angle - (PI / 2.0)).sin();
    let offset_x = offset * angle.cos();
    let offset_y = offset * angle.sin();
    let x_start = x - width_x * ((cols / 2) as f32) + width_x * ((((cols + 1) % 2) as f32) / 2.0);
    let y_start = y - width_y * ((cols / 2) as f32) + width_y * ((((cols + 1) % 2) as f32) / 2.0);    
    (0..rows)
	.map(|i| Point2::new(x_start + width_x * (i as f32) + ((i % 2) as f32) * offset_x,
			     y_start + width_y * (i as f32) + ((i % 2) as f32) * offset_y))
	.fold([].to_vec(), |mut points, point| {
	    points.append(&mut line_of_origins(cols, point.x, point.y, radius, angle));
	    points

//	points.append(&mut line_of_origins(cols, x_start + (i as f32) * x_offset_y + (((i % 2) as f32) * y_offset_x) / 2.0,
//				      y_start + (i as f32) * x_offset_x + (((i % 2) as f32) * y_offset_y) / 2.0,
//				      radius, angle));
//	points
    })
}

fn line_of_origins(num: i32, x: f32, y: f32, radius: f32, angle: f32) -> Vec<Point2> {
    let x_trans = radius * angle.cos();
    let y_trans = radius * angle.sin();
    let x_start = x - x_trans * ((num / 2) as f32) + x_trans * ((((num + 1) % 2) as f32) / 2.0);
    let y_start = y - y_trans * ((num / 2) as f32) + y_trans * ((((num + 1) % 2) as f32) / 2.0);    
    (0..num).map(|i| Point2::new(x_start + x_trans * (i as f32), y_start + y_trans * (i as f32))).collect()
}

fn draw_moving(app: &App, model: &Model, frame: Frame) {
    frame.clear(model.background_color);
    let draw = app.draw();
    let r = app.window_rect(); // => Rect
    let tower_first = if model.tower_left.height > model.tower_right.height { &model.tower_left }
    else { &model.tower_right };
    let tower_second = if model.tower_left.height < model.tower_right.height { &model.tower_left }
    else { &model.tower_right };

    let rotation_speed = app.elapsed_frames() as f32 * PI / 100.0;

    let relative_height = 0.05;
    let relative_width = 0.75;
    let relative_start = 0.20;
    let relative_spacing = 0.20;
    let stroke = 1.0;

    draw.rect()
	.stroke_weight(stroke)
	.stroke(BLACK)
	.x_y(tower_first.x_pos, r.bottom() + tower_first.height / 2.0)
	.w_h(tower_first.width, tower_first.height)
	.color(tower_first.color);

    for arm in 0..tower_first.arms {
	let arm_height = tower_first.height * relative_height;
	let arm_width = tower_first.width * relative_width;
	let angle_start = tower_first.width * relative_start;
	let spacing = tower_first.height * relative_spacing;
	let points = arrow(tower_first.x_pos + tower_first.width / 2.0,
			   tower_first.height - arm_height - (arm as f32) * spacing + r.bottom(),
			   arm_height,
			   arm_width,
			   angle_start);

	draw.polygon()
	    .stroke_weight(stroke)
	    .stroke(BLACK)
	    .points(rotate_points_xz(tower_first.x_pos, 0.0, rotation_speed, points.clone()))
	    .color(tower_first.color);

	draw.polygon()
	    .stroke_weight(stroke)
	    .stroke(BLACK)
	    .points(rotate_points_xz(tower_first.x_pos, 0.0, rotation_speed + PI, points.clone()))
	    .color(tower_first.color);
    }

    draw.rect()
	.x_y_z(tower_second.x_pos, r.bottom() + tower_second.height / 2.0, 0.0)
	.stroke_weight(stroke)
	.stroke(BLACK)
	.w_h(tower_second.width, tower_second.height)
	.color(tower_second.color);

    for arm in 0..tower_second.arms {
	let arm_height = tower_second.height * relative_height;
	let arm_width = tower_second.width * relative_width;
	let angle_start = tower_second.width * relative_start;
	let spacing = tower_second.height * relative_spacing;
	let points = arrow(tower_second.x_pos + tower_second.width / 2.0,
			   tower_second.height - arm_height - (arm as f32) * spacing + r.bottom(),
			   arm_height,
			   arm_width,
			   angle_start);

	draw.polygon()
	    .stroke_weight(stroke)
	    .stroke(BLACK)
	    .color(tower_second.color)
	    .points(rotate_points_xz(tower_second.x_pos, 0.0, rotation_speed, points.clone()));

	draw.polygon()
	    .stroke_weight(stroke)
	    .stroke(BLACK)
	    .color(tower_second.color)
	    .points(rotate_points_xz(tower_second.x_pos, 0.0, rotation_speed + PI, points.clone()));
    }


    // for point in grid(10.0, -100.0, 3, 3, 100.0, 12.5, 25.0, model.r) {
    // 	draw.rect().x_y(point.x, point.y).w_h(25.0, 100.0).rotate(model.r);
    // 	draw.rect().x_y(point.x, point.y).no_fill().stroke_color(BLACK).stroke_weight(2.5).w_h(25.0, 100.0).rotate(model.r);
    // }

//    for point in grid(10.0, -100.0, 3, 3, 25.0, 0.0, 25.0, model.r) {
//	draw.ellipse().x_y(point.x, point.y).w_h(25.0, 25.0).rotate(model.r);
//	draw.ellipse().x_y(point.x, point.y).no_fill().stroke_color(BLACK).stroke_weight(2.5).w_h(25.0, 25.0).rotate(model.r);
//    }

    draw.to_frame(app, &frame).unwrap();
}
