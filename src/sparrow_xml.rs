use std::fmt::Display;

use crate::{asprite_json::AspriteJson, util};

#[derive(Debug)]
pub struct TextureAtlas {
    pub image_path: String,
    pub frames: Vec<SubTexture>,
    pub animations: Vec<Animation>,
}

#[derive(Debug)]
pub struct Animation {
    name: String,
    start_idx: usize,
    end_idx: usize,
}

#[derive(Debug)]
pub struct SubTexture {
    pub name: String,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub frame_x: f64,
    pub frame_y: f64,
    pub frame_width: f64,
    pub frame_height: f64,
    duration: usize,
}

impl From<AspriteJson> for TextureAtlas {
    fn from(value: AspriteJson) -> Self {
        let image_path = value.meta.image;
        let mut frames: Vec<SubTexture> = value
            .frames
            .iter()
            .map(|(name, frame)| {
                let x = frame.frame.x;
                let width = frame.frame.w;
                let y = frame.frame.y;
                let height = frame.frame.h;
                SubTexture {
                    x,
                    y,
                    width,
                    height,
                    frame_y: frame.sprite_source_size.y,
                    frame_x: frame.sprite_source_size.x,
                    frame_height: frame.sprite_source_size.h,
                    frame_width: frame.sprite_source_size.w,
                    name: name.to_string(),
                    duration: frame.duration / 100,
                }
            })
            .collect();
        frames.sort_by_key(|frame| util::extract_number(&frame.name));
        let animations: Vec<Animation> = value
            .meta
            .frame_tags
            .iter()
            .map(|tag| Animation {
                name: tag.name.clone(),
                start_idx: tag.from,
                end_idx: tag.to,
            })
            .collect();
        Self {
            image_path,
            frames,
            animations,
        }
    }
}

impl Display for TextureAtlas {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let comments =
            r#"<?xml version="1.0" encoding="utf-8"?>\n<!-- Converted from aseprite json -->\n<!-- Converter by DifficultBit2901 -->"#
                .replace("\\n", "\n");
        let header = format!(r#"<TextureAtlas imagePath="{}">"#, self.image_path);
        let footer = "</TextureAtlas>";
        let mut frames: Vec<String> = Vec::new();

        for animation in &self.animations {
            let mut anim_idx = 0;
            for i in animation.start_idx..=animation.end_idx {
                let frame = &self.frames[i];
                for _ in 0..frame.duration {
                    let frame_name = format!("{}{:04}", &animation.name, anim_idx);
                    let frame_item = format!(
                    "\t<SubTexture name=\"{}\" width=\"{}\" height=\"{}\" x=\"{}\" y=\"{}\" frameWidth=\"{}\" frameHeight=\"{}\" frameX=\"{}\" frameY=\"{}\" />",
                    frame_name, frame.width, frame.height, frame.x, frame.y, frame.frame_width, frame.frame_height, frame.frame_x, frame.frame_y
                );
                    frames.push(frame_item);
                    anim_idx += 1;
                }
            }
        }

        let frame_text = frames.join("\n");
        write!(f, "{comments}\n{header}\n{frame_text}\n{footer}")
    }
}

impl Display for SubTexture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"<SubTexture name="{}" x="{}" y="{}" width="{}" height="{}" frameWidth="{}" frameHeight="{}" frameX="{}" frameY="{}"/>"#,
            self.name,
            self.x,
            self.y,
            self.width,
            self.height,
            self.frame_width,
            self.frame_height,
            self.frame_x,
            self.frame_y
        )
    }
}
