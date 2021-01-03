use super::{
    img_ids::{Imgs, ImgsRot},
    item_imgs::{ItemImgs, ItemKey::Tool},
    Show, TEXT_COLOR, UI_HIGHLIGHT_0, UI_MAIN, XP_COLOR, HP_COLOR, CRITICAL_HP_COLOR,
};
use crate::{
    i18n::Localization,
    ui::{fonts::Fonts, ImageFrame, Tooltip, TooltipManager, Tooltipable},
};
use conrod_core::{
    color,
    widget::{self, Button, Image, Rectangle, Text},
    widget_ids, Color, Colorable, Labelable, Positionable, Sizeable, Widget, WidgetCommon,
};

use client::{self, Client};
use common::comp::{
    item::tool::ToolKind,
    skills::{self, Skill},
    Stats,
};
use inline_tweak::*;

widget_ids! {
    pub struct Ids {
        frame,
        bg,
        icon,
        close,
        title,
        content_align,
        exp_bar_bg,
        exp_bar_frame,
        exp_bar_content_align,
        exp_bar_content,
        exp_bar_rank,
        exp_bar_txt,
        tree_title_txt,
        lock_imgs[],
        available_pts_txt,
        weapon_imgs[],
        weapon_btns[],
        skills_top_l_align,
        skills_top_r_align,
        skills_bot_l_align,
        skills_bot_r_align,
        skills_top_l[],
        skills_top_r[],
        skills_bot_l[],
        skills_bot_r[],
        sword_render,
        skill_sword_combo_0,
        skill_sword_combo_1,
        skill_sword_combo_2,
        skill_sword_combo_3,
        skill_sword_dash_0,
        skill_sword_dash_1,
        skill_sword_dash_2,
        skill_sword_dash_3,
        skill_sword_dash_4,
        skill_sword_dash_5,
        skill_sword_spin_0,
        skill_sword_spin_1,
        skill_sword_spin_2,
        skill_sword_spin_3,
        skill_sword_spin_4,
        skill_sword_passive_0,
        axe_render,
        skill_axe_combo_0,
        skill_axe_combo_1,
        skill_axe_combo_2,
        skill_axe_combo_3,
        skill_axe_spin_0,
        skill_axe_spin_1,
        skill_axe_spin_2,
        skill_axe_spin_3,
        skill_axe_spin_4,
        skill_axe_leap_0,
        skill_axe_leap_1,
        skill_axe_leap_2,
        skill_axe_leap_3,
        skill_axe_leap_4,
        hammer_render,
        skill_hammer_combo_0,
        skill_hammer_combo_1,
        skill_hammer_combo_2,
        skill_hammer_combo_3,
        skill_hammer_charged_0,
        skill_hammer_charged_1,
        skill_hammer_charged_2,
        skill_hammer_charged_3,
        skill_hammer_leap_0,
        skill_hammer_leap_1,
        skill_hammer_leap_2,
        skill_hammer_leap_3,
        skill_hammer_leap_4,
        skill_hammer_leap_5,
        bow_render,
        skill_bow_basic_0,
        skill_bow_basic_1,
        skill_bow_charged_0,
        skill_bow_charged_1,
        skill_bow_charged_2,
        skill_bow_charged_3,
        skill_bow_charged_4,
        skill_bow_charged_5,
        skill_bow_repeater_0,
        skill_bow_repeater_1,
        skill_bow_repeater_2,
        skill_bow_repeater_3,
        skill_bow_repeater_4,
        skill_bow_passive_0,
        staff_render,
        skill_staff_basic_0,
        skill_staff_basic_1,
        skill_staff_basic_2,
        skill_staff_basic_3,
        skill_staff_beam_0,
        skill_staff_beam_1,
        skill_staff_beam_2,
        skill_staff_beam_3,
        skill_staff_shockwave_0,
        skill_staff_shockwave_1,
        skill_staff_shockwave_2,
        skill_staff_shockwave_3,
        skill_staff_shockwave_4,
        sceptre_render,
        skill_sceptre_beam_0,
        skill_sceptre_beam_1,
        skill_sceptre_beam_2,
        skill_sceptre_beam_3,
        skill_sceptre_beam_4,
        skill_sceptre_beam_5,
        skill_sceptre_bomb_0,
        skill_sceptre_bomb_1,
        skill_sceptre_bomb_2,
        skill_sceptre_bomb_3,
        skill_sceptre_bomb_4,
        general_combat_render_0,
        general_combat_render_1,
        skill_general_stat_0,
        skill_general_stat_1,
        skill_general_tree_0,
        skill_general_tree_1,
        skill_general_tree_2,
        skill_general_tree_3,
        skill_general_tree_4,
        skill_general_tree_5,
        skill_general_roll_0,
        skill_general_roll_1,
        skill_general_roll_2,
        skill_general_roll_3,
    }
}

#[derive(WidgetCommon)]
pub struct Diary<'a> {
    show: &'a Show,
    _client: &'a Client,
    stats: &'a Stats,

    imgs: &'a Imgs,
    item_imgs: &'a ItemImgs,
    fonts: &'a Fonts,
    localized_strings: &'a Localization,
    rot_imgs: &'a ImgsRot,
    tooltip_manager: &'a mut TooltipManager,
    pulse: f32,

    #[conrod(common_builder)]
    common: widget::CommonBuilder,
    created_btns_top_l: usize,
    created_btns_top_r: usize,
    created_btns_bot_l: usize,
    created_btns_bot_r: usize,
    hovering_exp_bar: bool,
}

impl<'a> Diary<'a> {
    pub fn new(
        show: &'a Show,
        _client: &'a Client,
        stats: &'a Stats,
        imgs: &'a Imgs,
        item_imgs: &'a ItemImgs,
        fonts: &'a Fonts,
        localized_strings: &'a Localization,
        rot_imgs: &'a ImgsRot,
        tooltip_manager: &'a mut TooltipManager,
        pulse: f32,
    ) -> Self {
        Self {
            show,
            _client,
            stats,
            imgs,
            item_imgs,
            fonts,
            localized_strings,
            rot_imgs,
            tooltip_manager,
            pulse,
            common: widget::CommonBuilder::default(),
            created_btns_top_l: 0,
            created_btns_top_r: 0,
            created_btns_bot_l: 0,
            created_btns_bot_r: 0,
            hovering_exp_bar: false,
        }
    }
}

/*pub struct State {
    ids: Ids,
}*/

/*pub enum DiaryTab {
    SkillTrees,
    Achievements,
}*/

pub type SelectedSkillTree = skills::SkillGroupType;

const TREES: [&str; 7] = [
    "General Combat",
    "Sword",
    "Hammer",
    "Axe",
    "Sceptre",
    "Bow",
    "Fire Staff",
];

pub enum Event {
    Close,
    ChangeSkillTree(SelectedSkillTree),
    UnlockSkill(Skill),
}

impl<'a> Widget for Diary<'a> {
    type Event = Vec<Event>;
    type State = Ids;
    type Style = ();

    fn init_state(&self, id_gen: widget::id::Generator) -> Self::State { Ids::new(id_gen) }

    #[allow(clippy::unused_unit)] // TODO: Pending review in #587
    fn style(&self) -> Self::Style { () }

    fn update(mut self, args: widget::UpdateArgs<Self>) -> Self::Event {
        let widget::UpdateArgs {
            id: _, state, ui, ..
        } = args;
        let mut events = Vec::new();
        // Tooltips
        let diary_tooltip = Tooltip::new({
            // Edge images [t, b, r, l]
            // Corner images [tr, tl, br, bl]
            let edge = &self.rot_imgs.tt_side;
            let corner = &self.rot_imgs.tt_corner;
            ImageFrame::new(
                [edge.cw180, edge.none, edge.cw270, edge.cw90],
                [corner.none, corner.cw270, corner.cw90, corner.cw180],
                Color::Rgba(0.08, 0.07, 0.04, 1.0),
                5.0,
            )
        })
        .title_font_size(self.fonts.cyri.scale(15))
        .parent(ui.window)
        .desc_font_size(self.fonts.cyri.scale(12))
        .font_id(self.fonts.cyri.conrod_id)
        .desc_text_color(TEXT_COLOR);
        let sel_tab = &self.show.skilltreetab;
        let frame_ani = (self.pulse * 4.0/* speed factor */).cos() * 0.5 + 0.8; //Animation timer
        // Frame
        Image::new(self.imgs.diary_bg)
            .w_h(1202.0, 886.0)
            .mid_top_with_margin_on(ui.window, 5.0)
            .color(Some(UI_MAIN))
            .set(state.bg, ui);

        Image::new(self.imgs.diary_frame)
            .w_h(1202.0, 886.0)
            .middle_of(state.bg)
            .color(Some(UI_HIGHLIGHT_0))
            .set(state.frame, ui);

        // Icon
        Image::new(self.imgs.spellbook_button)
            .w_h(30.0, 27.0)
            .top_left_with_margins_on(state.frame, 8.0, 8.0)
            .set(state.icon, ui);

        // X-Button
        if Button::image(self.imgs.close_button)
            .w_h(24.0, 25.0)
            .hover_image(self.imgs.close_btn_hover)
            .press_image(self.imgs.close_btn_press)
            .top_right_with_margins_on(state.frame, 0.0, 0.0)
            .set(state.close, ui)
            .was_clicked()
        {
            events.push(Event::Close);
        }

        // Title
        Text::new(&self.localized_strings.get("hud.diary"))
            .mid_top_with_margin_on(state.frame, 3.0)
            .font_id(self.fonts.cyri.conrod_id)
            .font_size(self.fonts.cyri.scale(29))
            .color(TEXT_COLOR)
            .set(state.title, ui);

        // Content Alignment
        Rectangle::fill_with([599.0 * 2.0, 419.0 * 2.0], color::TRANSPARENT)
            .mid_top_with_margin_on(state.frame, 46.0)
            .set(state.content_align, ui);

        // Contents

        // Skill Trees

        // Skill Tree Selection
        state.update(|s| {
            s.weapon_btns
                .resize(TREES.len(), &mut ui.widget_id_generator())
        });
        state.update(|s| {
            s.weapon_imgs
                .resize(TREES.len(), &mut ui.widget_id_generator())
        });
        state.update(|s| {
            s.lock_imgs
                .resize(TREES.len(), &mut ui.widget_id_generator())
        });
        for i in TREES.iter().copied().enumerate() {
            let locked = !skill_tree_from_str(i.1)
                .map_or(false, |st| self.stats.skill_set.contains_skill_group(st));

            // Background weapon image
            let img = Image::new(match i.1 {
                "General Combat" => self.imgs.swords_crossed,
                "Sword" => self.imgs.sword,
                "Hammer" => self.imgs.hammer,
                "Axe" => self.imgs.axe,
                "Sceptre" => self.imgs.sceptre,
                "Bow" => self.imgs.bow,
                "Fire Staff" => self.imgs.staff,
                _ => self.imgs.nothing,
            });

            let img = if i.0 == 0 {
                img.top_left_with_margins_on(state.content_align, tweak!(10.0), tweak!(5.0))
            } else {
                img.down_from(state.weapon_btns[i.0 - 1], tweak!(5.0))
            };
            let tooltip_txt = if !locked { "" } else { "Not yet unlocked" };
            img.w_h(tweak!(50.0), tweak!(50.0))
                .set(state.weapon_imgs[i.0], ui);
            // Lock Image
            if locked {
                Image::new(self.imgs.lock)
                    .w_h(50.0, 50.0)
                    .middle_of(state.weapon_imgs[i.0])
                    .graphics_for(state.weapon_imgs[i.0])
                    .color(Some(Color::Rgba(1.0, 1.0, 1.0, tweak!(0.8))))
                    .set(state.lock_imgs[i.0], ui);
            }
            // Weapon icons
            let available_pts = skill_tree_from_str(i.1)
                .map_or(false, |st| self.stats.skill_set.get_available_sp(st) > 0);
            self.stats.skill_set.get_available_sp(*sel_tab);
            if Button::image(
                if skill_tree_from_str(i.1).map_or(false, |st| st == *sel_tab || available_pts) {
                    self.imgs.wpn_icon_border_pressed
                } else {
                    self.imgs.wpn_icon_border
                },
            )
            .w_h(tweak!(50.0), tweak!(50.0))
            .hover_image(match skill_tree_from_str(i.1).map(|st| st == *sel_tab) {
                Some(true) => self.imgs.wpn_icon_border_pressed,
                Some(false) => self.imgs.wpn_icon_border_mo,
                None => self.imgs.wpn_icon_border,
            })
            .press_image(match skill_tree_from_str(i.1).map(|st| st == *sel_tab) {
                Some(true) => self.imgs.wpn_icon_border_pressed,
                Some(false) => self.imgs.wpn_icon_border_press,
                None => self.imgs.wpn_icon_border,
            })
            .middle_of(state.weapon_imgs[i.0])
            .image_color(
                if skill_tree_from_str(i.1).map_or(false, |st| st != *sel_tab && available_pts) {
                    Color::Rgba(0.92, 0.76, 0.0, frame_ani)
                } else {
                    TEXT_COLOR
                },
            )
            .with_tooltip(
                self.tooltip_manager,
                i.1,
                &tooltip_txt,
                &diary_tooltip,
                TEXT_COLOR,
            )
            .set(state.weapon_btns[i.0], ui)
            .was_clicked()
            {
                events.push(skill_tree_from_str(i.1).map_or(Event::Close, Event::ChangeSkillTree))
            }
        }
        // Exp Bars and Rank Display
        let current_exp = self.stats.skill_set.get_experience(*sel_tab) as f64;
        let max_exp = sel_tab.skill_point_cost() as f64;
        let exp_percentage = current_exp / max_exp;
        let rank = self.stats.skill_set.get_earned_sp(*sel_tab);
        let rank_txt = format!("{}", rank);
        let exp_txt = format!("{}/{}", current_exp, max_exp);
        let available_pts = self.stats.skill_set.get_available_sp(*sel_tab);
        let available_pts_txt = format!("{} SP available!", available_pts);
        Image::new(self.imgs.diary_exp_bg)
            .w_h(480.0, 76.0)
            .mid_bottom_with_margin_on(state.content_align, tweak!(10.0))
            .set(state.exp_bar_bg, ui);
        Rectangle::fill_with([400.0, 40.0], color::TRANSPARENT)
            .top_left_with_margins_on(state.exp_bar_bg, 32.0, 40.0)
            .set(state.exp_bar_content_align, ui);
        Image::new(self.imgs.bar_content)
            .w_h(400.0 * exp_percentage, 40.0)
            .top_left_with_margins_on(state.exp_bar_content_align, 0.0, 0.0)
            .color(Some(XP_COLOR))
            .set(state.exp_bar_content, ui);
        Image::new(self.imgs.diary_exp_frame)
            .w_h(480.0, 76.0)
            .color(Some(UI_HIGHLIGHT_0))
            .middle_of(state.exp_bar_bg)
            .set(state.exp_bar_frame, ui);
        // Show EXP bar text on hover
        self.hovering_exp_bar = ui
            .widget_input(state.exp_bar_frame)
            .mouse()
            .map_or(false, |m| m.is_over());
        if self.hovering_exp_bar {
            Text::new(&exp_txt)
                .mid_top_with_margin_on(state.exp_bar_frame, tweak!(47.0))
                .font_id(self.fonts.cyri.conrod_id)
                .font_size(self.fonts.cyri.scale(tweak!(14)))
                .color(TEXT_COLOR)
                .graphics_for(state.exp_bar_frame)
                .set(state.exp_bar_txt, ui);
        }
        Text::new(&rank_txt)
            .mid_top_with_margin_on(state.exp_bar_frame, tweak!(5.0))
            .font_id(self.fonts.cyri.conrod_id)
            .font_size(self.fonts.cyri.scale(tweak!(28)))
            .color(TEXT_COLOR)
            .set(state.exp_bar_rank, ui);
        if available_pts > 0 {
            Text::new(&available_pts_txt)
                .mid_top_with_margin_on(state.content_align, tweak!(42.0))
                .font_id(self.fonts.cyri.conrod_id)
                .font_size(self.fonts.cyri.scale(tweak!(28)))
                .color(Color::Rgba(0.92, 0.76, 0.0, frame_ani))
                .set(state.available_pts_txt, ui);
        }
        let tree_title = match sel_tab {
            SelectedSkillTree::General => "General Combat",
            SelectedSkillTree::Weapon(ToolKind::Sword) => "Sword",
            SelectedSkillTree::Weapon(ToolKind::Hammer) => "Hammer",
            SelectedSkillTree::Weapon(ToolKind::Axe) => "Axe",
            SelectedSkillTree::Weapon(ToolKind::Sceptre) => "Healing Sceptre",
            SelectedSkillTree::Weapon(ToolKind::Bow) => "Bow",
            SelectedSkillTree::Weapon(ToolKind::Staff) => "Fire Staff",
            _ => "Unknown",
        };
        Text::new(&tree_title)
            .mid_top_with_margin_on(state.content_align, tweak!(2.0))
            .font_id(self.fonts.cyri.conrod_id)
            .font_size(self.fonts.cyri.scale(tweak!(34)))
            .color(TEXT_COLOR)
            .set(state.tree_title_txt, ui);
        // Skill Trees
        // Alignment Placing
        let x = tweak!(200.0);
        let y = tweak!(100.0);
        // Alignment rectangles for skills
        Rectangle::fill_with([124.0 * 2.0, 124.0 * 2.0], color::TRANSPARENT)
            .top_left_with_margins_on(state.content_align, y, x)
            .set(state.skills_top_l_align, ui);
        Rectangle::fill_with([124.0 * 2.0, 124.0 * 2.0], color::TRANSPARENT)
            .top_right_with_margins_on(state.content_align, y, x)
            .set(state.skills_top_r_align, ui);
        Rectangle::fill_with([124.0 * 2.0, 124.0 * 2.0], color::TRANSPARENT)
            .bottom_left_with_margins_on(state.content_align, y, x)
            .set(state.skills_bot_l_align, ui);
        Rectangle::fill_with([124.0 * 2.0, 124.0 * 2.0], color::TRANSPARENT)
            .bottom_right_with_margins_on(state.content_align, y, x)
            .set(state.skills_bot_r_align, ui);
        // Number of skills per rectangle per weapon, start counting at 0
        // Maximum of 9 skills/8 indices
        let skills_top_l = match sel_tab {
            SelectedSkillTree::General => 2,
            SelectedSkillTree::Weapon(ToolKind::Sword) => 4,
            SelectedSkillTree::Weapon(ToolKind::Axe) => 4,
            SelectedSkillTree::Weapon(ToolKind::Hammer) => 4,
            SelectedSkillTree::Weapon(ToolKind::Bow) => 2,
            SelectedSkillTree::Weapon(ToolKind::Staff) => 4,
            SelectedSkillTree::Weapon(ToolKind::Sceptre) => 6,
            _ => 0,
        };
        let skills_top_r = match sel_tab {
            SelectedSkillTree::General => 6,
            SelectedSkillTree::Weapon(ToolKind::Sword) => 6,
            SelectedSkillTree::Weapon(ToolKind::Axe) => 5,
            SelectedSkillTree::Weapon(ToolKind::Hammer) => 4,
            SelectedSkillTree::Weapon(ToolKind::Bow) => 6,
            SelectedSkillTree::Weapon(ToolKind::Staff) => 4,
            SelectedSkillTree::Weapon(ToolKind::Sceptre) => 5,
            _ => 0,
        };
        let skills_bot_l = match sel_tab {
            SelectedSkillTree::General => 4,
            SelectedSkillTree::Weapon(ToolKind::Sword) => 5,
            SelectedSkillTree::Weapon(ToolKind::Axe) => 5,
            SelectedSkillTree::Weapon(ToolKind::Hammer) => 6,
            SelectedSkillTree::Weapon(ToolKind::Bow) => 5,
            SelectedSkillTree::Weapon(ToolKind::Staff) => 5,
            _ => 0,
        };
        let skills_bot_r = match sel_tab {
            SelectedSkillTree::Weapon(ToolKind::Sword) => 1,
            SelectedSkillTree::Weapon(ToolKind::Bow) => 1,
            _ => 0,
        };
        // Update widget id array len
        state.update(|s| {
            s.skills_top_l
                .resize(skills_top_l, &mut ui.widget_id_generator())
        });
        state.update(|s| {
            s.skills_top_r
                .resize(skills_top_r, &mut ui.widget_id_generator())
        });
        state.update(|s| {
            s.skills_bot_l
                .resize(skills_bot_l, &mut ui.widget_id_generator())
        });
        state.update(|s| {
            s.skills_bot_r
                .resize(skills_bot_r, &mut ui.widget_id_generator())
        });
        // Create Background Images to place skill icons on them later
        // Create central skill first, others around it:
        //
        //        5 1 6
        //        3 0 4
        //        8 2 7
        //
        //
        // TOP-LEFT Skills
        while self.created_btns_top_l < skills_top_l {
            let mut img = Button::image(self.imgs.wpn_icon_border).w_h(80.0, 80.0);
            match self.created_btns_top_l {
                0 => img = img.middle_of(state.skills_top_l_align), // Central Skill
                1 => img = img.up_from(state.skills_top_l[0], 4.0), // 12:00
                2 => img = img.down_from(state.skills_top_l[0], 4.0), // 6:00
                3 => img = img.left_from(state.skills_top_l[0], 4.0), // 3:00
                4 => img = img.right_from(state.skills_top_l[0], 4.0), // 9:00
                5 => img = img.top_left_with_margins_on(state.skills_top_l[0], -84.0, -84.0), /* 10:30 */
                6 => img = img.top_right_with_margins_on(state.skills_top_l[0], -84.0, -84.0), /* 1:30 */
                7 => img = img.bottom_left_with_margins_on(state.skills_top_l[0], -84.0, -84.0), /* 4:30 */
                8 => img = img.bottom_right_with_margins_on(state.skills_top_l[0], -84.0, -84.0), /* 7:30 */
                _ => {},
            }
            img.set(state.skills_top_l[self.created_btns_top_l], ui);
            self.created_btns_top_l += 1;
        }
        // TOP-RIGHT Skills
        while self.created_btns_top_r < skills_top_r {
            let mut img = Button::image(self.imgs.wpn_icon_border).w_h(80.0, 80.0);
            match self.created_btns_top_r {
                0 => img = img.middle_of(state.skills_top_r_align), // Central Skill
                1 => img = img.up_from(state.skills_top_r[0], 4.0), // 12:00
                2 => img = img.down_from(state.skills_top_r[0], 4.0), // 6:00
                3 => img = img.left_from(state.skills_top_r[0], 4.0), // 3:00
                4 => img = img.right_from(state.skills_top_r[0], 4.0), // 9:00
                5 => img = img.top_left_with_margins_on(state.skills_top_r[0], -84.0, -84.0), /* 10:30 */
                6 => img = img.top_right_with_margins_on(state.skills_top_r[0], -84.0, -84.0), /* 1:30 */
                7 => img = img.bottom_left_with_margins_on(state.skills_top_r[0], -84.0, -84.0), /* 4:30 */
                8 => img = img.bottom_right_with_margins_on(state.skills_top_r[0], -84.0, -84.0), /* 7:30 */
                _ => {},
            }
            img.set(state.skills_top_r[self.created_btns_top_r], ui);
            self.created_btns_top_r += 1;
        }
        // BOTTOM-LEFT Skills
        while self.created_btns_bot_l < skills_bot_l {
            let mut img = Button::image(self.imgs.wpn_icon_border).w_h(80.0, 80.0);
            match self.created_btns_bot_l {
                0 => img = img.middle_of(state.skills_bot_l_align), // Central Skill
                1 => img = img.up_from(state.skills_bot_l[0], 4.0), // 12:00
                2 => img = img.down_from(state.skills_bot_l[0], 4.0), // 6:00
                3 => img = img.left_from(state.skills_bot_l[0], 4.0), // 3:00
                4 => img = img.right_from(state.skills_bot_l[0], 4.0), // 9:00
                5 => img = img.top_left_with_margins_on(state.skills_bot_l[0], -84.0, -84.0), /* 10:30 */
                6 => img = img.top_right_with_margins_on(state.skills_bot_l[0], -84.0, -84.0), /* 1:30 */
                7 => img = img.bottom_left_with_margins_on(state.skills_bot_l[0], -84.0, -84.0), /* 4:30 */
                8 => img = img.bottom_right_with_margins_on(state.skills_bot_l[0], -84.0, -84.0), /* 7:30 */
                _ => {},
            }
            img.set(state.skills_bot_l[self.created_btns_bot_l], ui);
            self.created_btns_bot_l += 1;
        }
        // BOTTOM-RIGHT Skills
        while self.created_btns_bot_r < skills_bot_r {
            let mut btn = Image::new(self.imgs.wpn_icon_border).w_h(80.0, 80.0);
            match self.created_btns_bot_r {
                0 => btn = btn.middle_of(state.skills_bot_r_align), // Central Skill
                1 => btn = btn.up_from(state.skills_bot_r[0], 4.0), // 12:00
                2 => btn = btn.down_from(state.skills_bot_r[0], 4.0), // 6:00
                3 => btn = btn.left_from(state.skills_bot_r[0], 4.0), // 3:00
                4 => btn = btn.right_from(state.skills_bot_r[0], 4.0), // 9:00
                5 => btn = btn.top_left_with_margins_on(state.skills_bot_r[0], -84.0, -84.0), /* 10:30 */
                6 => btn = btn.top_right_with_margins_on(state.skills_bot_r[0], -84.0, -84.0), /* 1:30 */
                7 => btn = btn.bottom_left_with_margins_on(state.skills_bot_r[0], -84.0, -84.0), /* 4:30 */
                8 => btn = btn.bottom_right_with_margins_on(state.skills_bot_r[0], -84.0, -84.0), /* 7:30 */
                _ => {},
            }
            btn.set(state.skills_bot_r[self.created_btns_bot_r], ui);
            self.created_btns_bot_r += 1;
        }
        // Skill-Icons and Functionality
        // Art dimensions
        let art_size = [tweak!(320.0), tweak!(320.0)];
        let skills = &self.stats.skill_set.skills;
        match sel_tab {
            SelectedSkillTree::General => {
                use skills::{GeneralSkill::*, RollSkill::*, SkillGroupType::*};
                use ToolKind::*;
                // General Combat
                Image::new(
                    self.item_imgs
                        .img_id_or_not_found_img(Tool("example_general_combat_left".to_string())),
                )
                .wh(art_size)
                .middle_of(state.content_align)
                .color(Some(Color::Rgba(1.0, 1.0, 1.0, tweak!(1.0))))
                .set(state.general_combat_render_0, ui);
                Image::new(
                    self.item_imgs
                        .img_id_or_not_found_img(Tool("example_general_combat_right".to_string())),
                )
                .wh(art_size)
                .middle_of(state.general_combat_render_0)
                .color(Some(Color::Rgba(1.0, 1.0, 1.0, tweak!(1.0))))
                .set(state.general_combat_render_1, ui);
                // Top Left skills
                //        5 1 6
                //        3 0 4
                //        8 2 7
                let skill = Skill::General(HealthIncrease);
                if Button::image(self.imgs.health_plus_skill)
                    .w_h(tweak!(74.0), tweak!(74.0))
                    .middle_of(state.skills_top_l[0])
                    .label(&format!(
                        "{}/{}",
                        skills.get(&skill).copied().map_or(0, |l| l.unwrap_or(1)),
                        skill.get_max_level().unwrap_or(1)
                    ))
                    .label_y(conrod_core::position::Relative::Scalar(tweak!(-28.0)))
                    .label_x(conrod_core::position::Relative::Scalar(tweak!(32.0)))
                    .label_color(TEXT_COLOR)
                    .label_font_size(self.fonts.cyri.scale(tweak!(16)))
                    .label_font_id(self.fonts.cyri.conrod_id)
                    .with_tooltip(
                        self.tooltip_manager,
                        "Increase Health",
                        "Increases health",
                        &diary_tooltip,
                        TEXT_COLOR,
                    )
                    .set(state.skill_general_stat_0, ui)
                    .was_clicked()
                {
                    events.push(Event::UnlockSkill(skill));
                };
                let skill = Skill::General(EnergyIncrease);
                if Button::image(self.imgs.stamina_plus_skill)
                    .w_h(tweak!(74.0), tweak!(74.0))
                    .middle_of(state.skills_top_l[1])
                    .label(&format!(
                        "{}/{}",
                        skills.get(&skill).copied().map_or(0, |l| l.unwrap_or(1)),
                        skill.get_max_level().unwrap_or(1)
                    ))
                    .label_y(conrod_core::position::Relative::Scalar(tweak!(-28.0)))
                    .label_x(conrod_core::position::Relative::Scalar(tweak!(32.0)))
                    .label_color(TEXT_COLOR)
                    .label_font_size(self.fonts.cyri.scale(tweak!(16)))
                    .label_font_id(self.fonts.cyri.conrod_id)
                    .with_tooltip(
                        self.tooltip_manager,
                        "Increase Energy",
                        "Increases energy",
                        &diary_tooltip,
                        TEXT_COLOR,
                    )
                    .set(state.skill_general_stat_1, ui)
                    .was_clicked()
                {
                    events.push(Event::UnlockSkill(skill));
                };
                // Top right skills
                let skill = Skill::UnlockGroup(Weapon(Sword));
                if Button::image(self.imgs.unlock_sword_skill)
                    .w_h(tweak!(74.0), tweak!(74.0))
                    .middle_of(state.skills_top_r[0])
                    .label(&format!(
                        "{}/{}",
                        skills.get(&skill).copied().map_or(0, |l| l.unwrap_or(1)),
                        skill.get_max_level().unwrap_or(1)
                    ))
                    .label_y(conrod_core::position::Relative::Scalar(tweak!(-28.0)))
                    .label_x(conrod_core::position::Relative::Scalar(tweak!(32.0)))
                    .label_color(TEXT_COLOR)
                    .label_font_size(self.fonts.cyri.scale(tweak!(16)))
                    .label_font_id(self.fonts.cyri.conrod_id)
                    .with_tooltip(
                        self.tooltip_manager,
                        "Unlock Sword",
                        "Unlocks sword skill tree",
                        &diary_tooltip,
                        TEXT_COLOR,
                    )
                    .set(state.skill_general_tree_0, ui)
                    .was_clicked()
                {
                    events.push(Event::UnlockSkill(skill));
                };
                let skill = Skill::UnlockGroup(Weapon(Axe));
                if Button::image(self.imgs.unlock_axe_skill)
                    .w_h(tweak!(74.0), tweak!(74.0))
                    .middle_of(state.skills_top_r[1])
                    .label(&format!(
                        "{}/{}",
                        skills.get(&skill).copied().map_or(0, |l| l.unwrap_or(1)),
                        skill.get_max_level().unwrap_or(1)
                    ))
                    .label_y(conrod_core::position::Relative::Scalar(tweak!(-28.0)))
                    .label_x(conrod_core::position::Relative::Scalar(tweak!(32.0)))
                    .label_color(TEXT_COLOR)
                    .label_font_size(self.fonts.cyri.scale(tweak!(16)))
                    .label_font_id(self.fonts.cyri.conrod_id)
                    .with_tooltip(
                        self.tooltip_manager,
                        "Unlock Axe",
                        "Unlocks axe skill tree",
                        &diary_tooltip,
                        TEXT_COLOR,
                    )
                    .set(state.skill_general_tree_1, ui)
                    .was_clicked()
                {
                    events.push(Event::UnlockSkill(skill));
                };
                let skill = Skill::UnlockGroup(Weapon(Hammer));
                if Button::image(self.imgs.unlock_hammer_skill)
                    .w_h(tweak!(74.0), tweak!(74.0))
                    .middle_of(state.skills_top_r[2])
                    .label(&format!(
                        "{}/{}",
                        skills.get(&skill).copied().map_or(0, |l| l.unwrap_or(1)),
                        skill.get_max_level().unwrap_or(1)
                    ))
                    .label_y(conrod_core::position::Relative::Scalar(tweak!(-28.0)))
                    .label_x(conrod_core::position::Relative::Scalar(tweak!(32.0)))
                    .label_color(TEXT_COLOR)
                    .label_font_size(self.fonts.cyri.scale(tweak!(16)))
                    .label_font_id(self.fonts.cyri.conrod_id)
                    .with_tooltip(
                        self.tooltip_manager,
                        "Unlock Hammer",
                        "Unlocks hammer skill tree",
                        &diary_tooltip,
                        TEXT_COLOR,
                    )
                    .set(state.skill_general_tree_2, ui)
                    .was_clicked()
                {
                    events.push(Event::UnlockSkill(skill));
                };
                let skill = Skill::UnlockGroup(Weapon(Bow));
                if Button::image(self.imgs.unlock_bow_skill)
                    .w_h(tweak!(74.0), tweak!(74.0))
                    .middle_of(state.skills_top_r[3])
                    .label(&format!(
                        "{}/{}",
                        skills.get(&skill).copied().map_or(0, |l| l.unwrap_or(1)),
                        skill.get_max_level().unwrap_or(1)
                    ))
                    .label_y(conrod_core::position::Relative::Scalar(tweak!(-28.0)))
                    .label_x(conrod_core::position::Relative::Scalar(tweak!(32.0)))
                    .label_color(TEXT_COLOR)
                    .label_font_size(self.fonts.cyri.scale(tweak!(16)))
                    .label_font_id(self.fonts.cyri.conrod_id)
                    .with_tooltip(
                        self.tooltip_manager,
                        "Unlock Bow",
                        "Unlocks bow skill tree",
                        &diary_tooltip,
                        TEXT_COLOR,
                    )
                    .set(state.skill_general_tree_3, ui)
                    .was_clicked()
                {
                    events.push(Event::UnlockSkill(skill));
                };
                let skill = Skill::UnlockGroup(Weapon(Staff));
                if Button::image(self.imgs.unlock_staff_skill0)
                    .w_h(tweak!(74.0), tweak!(74.0))
                    .middle_of(state.skills_top_r[4])
                    .label(&format!(
                        "{}/{}",
                        skills.get(&skill).copied().map_or(0, |l| l.unwrap_or(1)),
                        skill.get_max_level().unwrap_or(1)
                    ))
                    .label_y(conrod_core::position::Relative::Scalar(tweak!(-28.0)))
                    .label_x(conrod_core::position::Relative::Scalar(tweak!(32.0)))
                    .label_color(TEXT_COLOR)
                    .label_font_size(self.fonts.cyri.scale(tweak!(16)))
                    .label_font_id(self.fonts.cyri.conrod_id)
                    .with_tooltip(
                        self.tooltip_manager,
                        "Unlock Staff",
                        "Unlocks staff skill tree",
                        &diary_tooltip,
                        TEXT_COLOR,
                    )
                    .set(state.skill_general_tree_4, ui)
                    .was_clicked()
                {
                    events.push(Event::UnlockSkill(skill));
                };
                let skill = Skill::UnlockGroup(Weapon(Sceptre));
                if Button::image(self.imgs.unlock_sceptre_skill)
                    .w_h(tweak!(74.0), tweak!(74.0))
                    .middle_of(state.skills_top_r[5])
                    .label(&format!(
                        "{}/{}",
                        skills.get(&skill).copied().map_or(0, |l| l.unwrap_or(1)),
                        skill.get_max_level().unwrap_or(1)
                    ))
                    .label_y(conrod_core::position::Relative::Scalar(tweak!(-28.0)))
                    .label_x(conrod_core::position::Relative::Scalar(tweak!(32.0)))
                    .label_color(TEXT_COLOR)
                    .label_font_size(self.fonts.cyri.scale(tweak!(16)))
                    .label_font_id(self.fonts.cyri.conrod_id)
                    .with_tooltip(
                        self.tooltip_manager,
                        "Unlock Sceptre",
                        "Unlocks sceptre skill tree",
                        &diary_tooltip,
                        TEXT_COLOR,
                    )
                    .set(state.skill_general_tree_5, ui)
                    .was_clicked()
                {
                    events.push(Event::UnlockSkill(skill));
                };
                // Bottom left skills
                let skill = Skill::Roll(ImmuneMelee);
                if Button::image(self.imgs.swords_crossed)
                    .w_h(tweak!(74.0), tweak!(74.0))
                    .middle_of(state.skills_bot_l[0])
                    .label(&format!(
                        "{}/{}",
                        skills.get(&skill).copied().map_or(0, |l| l.unwrap_or(1)),
                        skill.get_max_level().unwrap_or(1)
                    ))
                    .label_y(conrod_core::position::Relative::Scalar(tweak!(-28.0)))
                    .label_x(conrod_core::position::Relative::Scalar(tweak!(32.0)))
                    .label_color(TEXT_COLOR)
                    .label_font_size(self.fonts.cyri.scale(tweak!(16)))
                    .label_font_id(self.fonts.cyri.conrod_id)
                    .with_tooltip(
                        self.tooltip_manager,
                        "Dodge",
                        "Ground-yeeting dodges melee attacks",
                        &diary_tooltip,
                        TEXT_COLOR,
                    )
                    .set(state.skill_general_roll_0, ui)
                    .was_clicked()
                {
                    events.push(Event::UnlockSkill(skill));
                };
                let skill = Skill::Roll(Cost);
                if Button::image(self.imgs.swords_crossed)
                    .w_h(tweak!(74.0), tweak!(74.0))
                    .middle_of(state.skills_bot_l[1])
                    .label(&format!(
                        "{}/{}",
                        skills.get(&skill).copied().map_or(0, |l| l.unwrap_or(1)),
                        skill.get_max_level().unwrap_or(1)
                    ))
                    .label_y(conrod_core::position::Relative::Scalar(tweak!(-28.0)))
                    .label_x(conrod_core::position::Relative::Scalar(tweak!(32.0)))
                    .label_color(TEXT_COLOR)
                    .label_font_size(self.fonts.cyri.scale(tweak!(16)))
                    .label_font_id(self.fonts.cyri.conrod_id)
                    .with_tooltip(
                        self.tooltip_manager,
                        "Cost",
                        "Decreases cost of ground-yeeting yourself",
                        &diary_tooltip,
                        TEXT_COLOR,
                    )
                    .set(state.skill_general_roll_1, ui)
                    .was_clicked()
                {
                    events.push(Event::UnlockSkill(skill));
                };
                let skill = Skill::Roll(Strength);
                if Button::image(self.imgs.swords_crossed)
                    .w_h(tweak!(74.0), tweak!(74.0))
                    .middle_of(state.skills_bot_l[2])
                    .label(&format!(
                        "{}/{}",
                        skills.get(&skill).copied().map_or(0, |l| l.unwrap_or(1)),
                        skill.get_max_level().unwrap_or(1)
                    ))
                    .label_y(conrod_core::position::Relative::Scalar(tweak!(-28.0)))
                    .label_x(conrod_core::position::Relative::Scalar(tweak!(32.0)))
                    .label_color(TEXT_COLOR)
                    .label_font_size(self.fonts.cyri.scale(tweak!(16)))
                    .label_font_id(self.fonts.cyri.conrod_id)
                    .with_tooltip(
                        self.tooltip_manager,
                        "Strength",
                        "Increases how far you ground-yeet yourself",
                        &diary_tooltip,
                        TEXT_COLOR,
                    )
                    .set(state.skill_general_roll_2, ui)
                    .was_clicked()
                {
                    events.push(Event::UnlockSkill(skill));
                };
                let skill = Skill::Roll(Duration);
                if Button::image(self.imgs.swords_crossed)
                    .w_h(tweak!(74.0), tweak!(74.0))
                    .middle_of(state.skills_bot_l[3])
                    .label(&format!(
                        "{}/{}",
                        skills.get(&skill).copied().map_or(0, |l| l.unwrap_or(1)),
                        skill.get_max_level().unwrap_or(1)
                    ))
                    .label_y(conrod_core::position::Relative::Scalar(tweak!(-28.0)))
                    .label_x(conrod_core::position::Relative::Scalar(tweak!(32.0)))
                    .label_color(TEXT_COLOR)
                    .label_font_size(self.fonts.cyri.scale(tweak!(16)))
                    .label_font_id(self.fonts.cyri.conrod_id)
                    .with_tooltip(
                        self.tooltip_manager,
                        "Duration",
                        "Increases for how long you ground-yeet yourself",
                        &diary_tooltip,
                        TEXT_COLOR,
                    )
                    .set(state.skill_general_roll_3, ui)
                    .was_clicked()
                {
                    events.push(Event::UnlockSkill(skill));
                };
            },
            SelectedSkillTree::Weapon(ToolKind::Sword) => {
                use skills::SwordSkill::*;
                // Sword
                Image::new(
                    self.item_imgs
                        .img_id_or_not_found_img(Tool("example_sword".to_string())),
                )
                .wh(art_size)
                .middle_of(state.content_align)
                .color(Some(Color::Rgba(1.0, 1.0, 1.0, tweak!(1.0))))
                .set(state.sword_render, ui);
                // Top Left skills
                //        5 1 6
                //        3 0 4
                //        8 2 7
                let skill = Skill::Sword(TsCombo);
                if Button::image(self.imgs.sword_whirlwind)
                    .w_h(tweak!(74.0), tweak!(74.0))
                    .middle_of(state.skills_top_l[0])
                    .label(&format!(
                        "{}/{}",
                        skills.get(&skill).copied().map_or(0, |l| l.unwrap_or(1)),
                        skill.get_max_level().unwrap_or(1)
                    ))
                    .label_y(conrod_core::position::Relative::Scalar(tweak!(-28.0)))
                    .label_x(conrod_core::position::Relative::Scalar(tweak!(32.0)))
                    .label_color(TEXT_COLOR)
                    .label_font_size(self.fonts.cyri.scale(tweak!(16)))
                    .label_font_id(self.fonts.cyri.conrod_id)
                    .with_tooltip(
                        self.tooltip_manager,
                        "Triple Strike Combo",
                        "Unlocks combo scaling on triple strike",
                        &diary_tooltip,
                        TEXT_COLOR,
                    )
                    .set(state.skill_sword_combo_0, ui)
                    .was_clicked()
                {
                    events.push(Event::UnlockSkill(skill));
                };
                let skill = Skill::Sword(TsDamage);
                if Button::image(self.imgs.sword_whirlwind)
                    .w_h(tweak!(74.0), tweak!(74.0))
                    .middle_of(state.skills_top_l[1])
                    .label(&format!(
                        "{}/{}",
                        skills.get(&skill).copied().map_or(0, |l| l.unwrap_or(1)),
                        skill.get_max_level().unwrap_or(1)
                    ))
                    .label_y(conrod_core::position::Relative::Scalar(tweak!(-28.0)))
                    .label_x(conrod_core::position::Relative::Scalar(tweak!(32.0)))
                    .label_color(TEXT_COLOR)
                    .label_font_size(self.fonts.cyri.scale(tweak!(16)))
                    .label_font_id(self.fonts.cyri.conrod_id)
                    .with_tooltip(
                        self.tooltip_manager,
                        "Triple Strike Damage",
                        "Increases damage scaling on triple strike",
                        &diary_tooltip,
                        TEXT_COLOR,
                    )
                    .set(state.skill_sword_combo_1, ui)
                    .was_clicked()
                {
                    events.push(Event::UnlockSkill(skill));
                };
                let skill = Skill::Sword(TsSpeed);
                if Button::image(self.imgs.sword_whirlwind)
                    .w_h(tweak!(74.0), tweak!(74.0))
                    .middle_of(state.skills_top_l[2])
                    .label(&format!(
                        "{}/{}",
                        skills.get(&skill).copied().map_or(0, |l| l.unwrap_or(1)),
                        skill.get_max_level().unwrap_or(1)
                    ))
                    .label_y(conrod_core::position::Relative::Scalar(tweak!(-28.0)))
                    .label_x(conrod_core::position::Relative::Scalar(tweak!(32.0)))
                    .label_color(TEXT_COLOR)
                    .label_font_size(self.fonts.cyri.scale(tweak!(16)))
                    .label_font_id(self.fonts.cyri.conrod_id)
                    .with_tooltip(
                        self.tooltip_manager,
                        "Triple Strike Speed",
                        "Increases attack speed scaling on triple strike",
                        &diary_tooltip,
                        TEXT_COLOR,
                    )
                    .set(state.skill_sword_combo_2, ui)
                    .was_clicked()
                {
                    events.push(Event::UnlockSkill(skill));
                };
                let skill = Skill::Sword(TsRegen);
                if Button::image(self.imgs.sword_whirlwind)
                    .w_h(tweak!(74.0), tweak!(74.0))
                    .middle_of(state.skills_top_l[3])
                    .label(&format!(
                        "{}/{}",
                        skills.get(&skill).copied().map_or(0, |l| l.unwrap_or(1)),
                        skill.get_max_level().unwrap_or(1)
                    ))
                    .label_y(conrod_core::position::Relative::Scalar(tweak!(-28.0)))
                    .label_x(conrod_core::position::Relative::Scalar(tweak!(32.0)))
                    .label_color(TEXT_COLOR)
                    .label_font_size(self.fonts.cyri.scale(tweak!(16)))
                    .label_font_id(self.fonts.cyri.conrod_id)
                    .with_tooltip(
                        self.tooltip_manager,
                        "Triple Strike Regen",
                        "Increases enery regen scaling on triple strike",
                        &diary_tooltip,
                        TEXT_COLOR,
                    )
                    .set(state.skill_sword_combo_3, ui)
                    .was_clicked()
                {
                    events.push(Event::UnlockSkill(skill));
                };
                // Top right skills
                let skill = Skill::Sword(DDamage);
                if Button::image(self.imgs.sword_whirlwind)
                    .w_h(tweak!(74.0), tweak!(74.0))
                    .middle_of(state.skills_top_r[0])
                    .label(&format!(
                        "{}/{}",
                        skills.get(&skill).copied().map_or(0, |l| l.unwrap_or(1)),
                        skill.get_max_level().unwrap_or(1)
                    ))
                    .label_y(conrod_core::position::Relative::Scalar(tweak!(-28.0)))
                    .label_x(conrod_core::position::Relative::Scalar(tweak!(32.0)))
                    .label_color(TEXT_COLOR)
                    .label_font_size(self.fonts.cyri.scale(tweak!(16)))
                    .label_font_id(self.fonts.cyri.conrod_id)
                    .with_tooltip(
                        self.tooltip_manager,
                        "Dash Damage",
                        "Increases initial damage of the dash",
                        &diary_tooltip,
                        TEXT_COLOR,
                    )
                    .set(state.skill_sword_dash_0, ui)
                    .was_clicked()
                {
                    events.push(Event::UnlockSkill(skill));
                };
                let skill = Skill::Sword(DDrain);
                if Button::image(self.imgs.sword_whirlwind)
                    .w_h(tweak!(74.0), tweak!(74.0))
                    .middle_of(state.skills_top_r[1])
                    .label(&format!(
                        "{}/{}",
                        skills.get(&skill).copied().map_or(0, |l| l.unwrap_or(1)),
                        skill.get_max_level().unwrap_or(1)
                    ))
                    .label_y(conrod_core::position::Relative::Scalar(tweak!(-28.0)))
                    .label_x(conrod_core::position::Relative::Scalar(tweak!(32.0)))
                    .label_color(TEXT_COLOR)
                    .label_font_size(self.fonts.cyri.scale(tweak!(16)))
                    .label_font_id(self.fonts.cyri.conrod_id)
                    .with_tooltip(
                        self.tooltip_manager,
                        "Dash Drain",
                        "Decreases the rate energy is drained while dashing",
                        &diary_tooltip,
                        TEXT_COLOR,
                    )
                    .set(state.skill_sword_dash_1, ui)
                    .was_clicked()
                {
                    events.push(Event::UnlockSkill(skill));
                };
                let skill = Skill::Sword(DCost);
                let prereqs_met = tweak!(true);
                let suff_pts = tweak!(false);
                let label_txt = &format!(      "{}/{}",
                    skills.get(&skill).copied().map_or(0, |l| l.unwrap_or(1)),
                    skill.get_max_level().unwrap_or(1));
                if Button::image(self.imgs.sword_whirlwind)
                    .w_h(tweak!(74.0), tweak!(74.0))
                    .middle_of(state.skills_top_r[2])
                    .label(if prereqs_met {&label_txt} else {""}
                    )
                    .label_y(conrod_core::position::Relative::Scalar(tweak!(-28.0)))
                    .label_x(conrod_core::position::Relative::Scalar(tweak!(32.0)))
                    .label_color(if suff_pts {HP_COLOR} else {CRITICAL_HP_COLOR})
                    .label_font_size(self.fonts.cyri.scale(tweak!(16)))
                    .label_font_id(self.fonts.cyri.conrod_id)
                    .image_color(if prereqs_met {TEXT_COLOR} else {Color::Rgba(0.41, 0.41, 0.41, tweak!(0.7))})
                    .with_tooltip(
                        self.tooltip_manager,
                        "Dash Cost",
                        "Decreases the initial cost of the dash",
                        &diary_tooltip,
                        TEXT_COLOR,
                    )
                    .set(state.skill_sword_dash_2, ui)
                    .was_clicked()
                {
                    events.push(Event::UnlockSkill(skill));
                };
                let skill = Skill::Sword(DSpeed);
                if Button::image(self.imgs.sword_whirlwind)
                    .w_h(tweak!(74.0), tweak!(74.0))
                    .middle_of(state.skills_top_r[3])
                    .label(&format!(
                        "{}/{}",
                        skills.get(&skill).copied().map_or(0, |l| l.unwrap_or(1)),
                        skill.get_max_level().unwrap_or(1)
                    ))
                    .label_y(conrod_core::position::Relative::Scalar(tweak!(-28.0)))
                    .label_x(conrod_core::position::Relative::Scalar(tweak!(32.0)))
                    .label_color(TEXT_COLOR)
                    .label_font_size(self.fonts.cyri.scale(tweak!(16)))
                    .label_font_id(self.fonts.cyri.conrod_id)
                    .with_tooltip(
                        self.tooltip_manager,
                        "Dash Speed",
                        "Increases how fast you go while dashing",
                        &diary_tooltip,
                        TEXT_COLOR,
                    )
                    .set(state.skill_sword_dash_3, ui)
                    .was_clicked()
                {
                    events.push(Event::UnlockSkill(skill));
                };
                let skill = Skill::Sword(DInfinite);
                if Button::image(self.imgs.sword_whirlwind)
                    .w_h(tweak!(74.0), tweak!(74.0))
                    .middle_of(state.skills_top_r[4])
                    .label(&format!(
                        "{}/{}",
                        skills.get(&skill).copied().map_or(0, |l| l.unwrap_or(1)),
                        skill.get_max_level().unwrap_or(1)
                    ))
                    .label_y(conrod_core::position::Relative::Scalar(tweak!(-28.0)))
                    .label_x(conrod_core::position::Relative::Scalar(tweak!(32.0)))
                    .label_color(TEXT_COLOR)
                    .label_font_size(self.fonts.cyri.scale(tweak!(16)))
                    .label_font_id(self.fonts.cyri.conrod_id)
                    .with_tooltip(
                        self.tooltip_manager,
                        "Dash Infinite",
                        "Allows you to dash for as long as you have energy",
                        &diary_tooltip,
                        TEXT_COLOR,
                    )
                    .set(state.skill_sword_dash_4, ui)
                    .was_clicked()
                {
                    events.push(Event::UnlockSkill(skill));
                };
                let skill = Skill::Sword(DScaling);
                if Button::image(self.imgs.sword_whirlwind)
                    .w_h(tweak!(74.0), tweak!(74.0))
                    .middle_of(state.skills_top_r[5])
                    .label(&format!(
                        "{}/{}",
                        skills.get(&skill).copied().map_or(0, |l| l.unwrap_or(1)),
                        skill.get_max_level().unwrap_or(1)
                    ))
                    .label_y(conrod_core::position::Relative::Scalar(tweak!(-28.0)))
                    .label_x(conrod_core::position::Relative::Scalar(tweak!(32.0)))
                    .label_color(TEXT_COLOR)
                    .label_font_size(self.fonts.cyri.scale(tweak!(16)))
                    .label_font_id(self.fonts.cyri.conrod_id)
                    .with_tooltip(
                        self.tooltip_manager,
                        "Dash Scaling",
                        "Increases how much the damage scales by over the course of the dash",
                        &diary_tooltip,
                        TEXT_COLOR,
                    )
                    .set(state.skill_sword_dash_5, ui)
                    .was_clicked()
                {
                    events.push(Event::UnlockSkill(skill));
                };
                // Bottom left skills
                let skill = Skill::Sword(SUnlockSpin);
                if Button::image(self.imgs.sword_whirlwind)
                    .w_h(tweak!(74.0), tweak!(74.0))
                    .middle_of(state.skills_bot_l[0])
                    .label(&format!(
                        "{}/{}",
                        skills.get(&skill).copied().map_or(0, |l| l.unwrap_or(1)),
                        skill.get_max_level().unwrap_or(1)
                    ))
                    .label_y(conrod_core::position::Relative::Scalar(tweak!(-28.0)))
                    .label_x(conrod_core::position::Relative::Scalar(tweak!(32.0)))
                    .label_color(TEXT_COLOR)
                    .label_font_size(self.fonts.cyri.scale(tweak!(16)))
                    .label_font_id(self.fonts.cyri.conrod_id)
                    .with_tooltip(
                        self.tooltip_manager,
                        "Spin Unlock",
                        "Unlocks the sword spin",
                        &diary_tooltip,
                        TEXT_COLOR,
                    )
                    .set(state.skill_sword_spin_0, ui)
                    .was_clicked()
                {
                    events.push(Event::UnlockSkill(skill));
                };
                let skill = Skill::Sword(SDamage);
                if Button::image(self.imgs.sword_whirlwind)
                    .w_h(tweak!(74.0), tweak!(74.0))
                    .middle_of(state.skills_bot_l[1])
                    .label(&format!(
                        "{}/{}",
                        skills.get(&skill).copied().map_or(0, |l| l.unwrap_or(1)),
                        skill.get_max_level().unwrap_or(1)
                    ))
                    .label_y(conrod_core::position::Relative::Scalar(tweak!(-28.0)))
                    .label_x(conrod_core::position::Relative::Scalar(tweak!(32.0)))
                    .label_color(TEXT_COLOR)
                    .label_font_size(self.fonts.cyri.scale(tweak!(16)))
                    .label_font_id(self.fonts.cyri.conrod_id)
                    .with_tooltip(
                        self.tooltip_manager,
                        "Spin Damage",
                        "Increases the damage done",
                        &diary_tooltip,
                        TEXT_COLOR,
                    )
                    .set(state.skill_sword_spin_1, ui)
                    .was_clicked()
                {
                    events.push(Event::UnlockSkill(skill));
                };
                let skill = Skill::Sword(SSpeed);
                if Button::image(self.imgs.sword_whirlwind)
                    .w_h(tweak!(74.0), tweak!(74.0))
                    .middle_of(state.skills_bot_l[2])
                    .label(&format!(
                        "{}/{}",
                        skills.get(&skill).copied().map_or(0, |l| l.unwrap_or(1)),
                        skill.get_max_level().unwrap_or(1)
                    ))
                    .label_y(conrod_core::position::Relative::Scalar(tweak!(-28.0)))
                    .label_x(conrod_core::position::Relative::Scalar(tweak!(32.0)))
                    .label_color(TEXT_COLOR)
                    .label_font_size(self.fonts.cyri.scale(tweak!(16)))
                    .label_font_id(self.fonts.cyri.conrod_id)
                    .with_tooltip(
                        self.tooltip_manager,
                        "Spin Speed",
                        "Increase the speed at which you spin",
                        &diary_tooltip,
                        TEXT_COLOR,
                    )
                    .set(state.skill_sword_spin_2, ui)
                    .was_clicked()
                {
                    events.push(Event::UnlockSkill(skill));
                };
                let skill = Skill::Sword(SCost);
                if Button::image(self.imgs.sword_whirlwind)
                    .w_h(tweak!(74.0), tweak!(74.0))
                    .middle_of(state.skills_bot_l[3])
                    .label(&format!(
                        "{}/{}",
                        skills.get(&skill).copied().map_or(0, |l| l.unwrap_or(1)),
                        skill.get_max_level().unwrap_or(1)
                    ))
                    .label_y(conrod_core::position::Relative::Scalar(tweak!(-28.0)))
                    .label_x(conrod_core::position::Relative::Scalar(tweak!(32.0)))
                    .label_color(TEXT_COLOR)
                    .label_font_size(self.fonts.cyri.scale(tweak!(16)))
                    .label_font_id(self.fonts.cyri.conrod_id)
                    .with_tooltip(
                        self.tooltip_manager,
                        "Spin Cost",
                        "Decreases the energy cost of each spin",
                        &diary_tooltip,
                        TEXT_COLOR,
                    )
                    .set(state.skill_sword_spin_3, ui)
                    .was_clicked()
                {
                    events.push(Event::UnlockSkill(skill));
                };
                let skill = Skill::Sword(SSpins);
                if Button::image(self.imgs.sword_whirlwind)
                    .w_h(tweak!(74.0), tweak!(74.0))
                    .middle_of(state.skills_bot_l[4])
                    .label(&format!(
                        "{}/{}",
                        skills.get(&skill).copied().map_or(0, |l| l.unwrap_or(1)),
                        skill.get_max_level().unwrap_or(1)
                    ))
                    .label_y(conrod_core::position::Relative::Scalar(tweak!(-28.0)))
                    .label_x(conrod_core::position::Relative::Scalar(tweak!(32.0)))
                    .label_color(TEXT_COLOR)
                    .label_font_size(self.fonts.cyri.scale(tweak!(16)))
                    .label_font_id(self.fonts.cyri.conrod_id)
                    .with_tooltip(
                        self.tooltip_manager,
                        "Spin Spins",
                        "Increases the number of times you can spin",
                        &diary_tooltip,
                        TEXT_COLOR,
                    )
                    .set(state.skill_sword_spin_4, ui)
                    .was_clicked()
                {
                    events.push(Event::UnlockSkill(skill));
                };
                // Bottom right skills
                let skill = Skill::Sword(InterruptingAttacks);
                if Button::image(self.imgs.sword_whirlwind)
                    .w_h(tweak!(74.0), tweak!(74.0))
                    .middle_of(state.skills_bot_r[0])
                    .label(&format!(
                        "{}/{}",
                        skills.get(&skill).copied().map_or(0, |l| l.unwrap_or(1)),
                        skill.get_max_level().unwrap_or(1)
                    ))
                    .label_y(conrod_core::position::Relative::Scalar(tweak!(-28.0)))
                    .label_x(conrod_core::position::Relative::Scalar(tweak!(32.0)))
                    .label_color(TEXT_COLOR)
                    .label_font_size(self.fonts.cyri.scale(tweak!(16)))
                    .label_font_id(self.fonts.cyri.conrod_id)
                    .with_tooltip(
                        self.tooltip_manager,
                        "Interrupting Attacks",
                        "Allows you to immediately cancel an attack with another attack",
                        &diary_tooltip,
                        TEXT_COLOR,
                    )
                    .set(state.skill_sword_passive_0, ui)
                    .was_clicked()
                {
                    events.push(Event::UnlockSkill(skill));
                };
            },
            SelectedSkillTree::Weapon(ToolKind::Axe) => {
                use skills::AxeSkill::*;
                // Axe
                Image::new(
                    self.item_imgs
                        .img_id_or_not_found_img(Tool("example_axe".to_string())),
                )
                .wh(art_size)
                .middle_of(state.content_align)
                .color(Some(Color::Rgba(1.0, 1.0, 1.0, tweak!(1.0))))
                .set(state.axe_render, ui);
                // Top Left skills
                //        5 1 6
                //        3 0 4
                //        8 2 7
                let skill = Skill::Axe(DsCombo);
                if Button::image(self.imgs.axespin)
                    .w_h(tweak!(74.0), tweak!(74.0))
                    .middle_of(state.skills_top_l[0])
                    .label(&format!(
                        "{}/{}",
                        skills.get(&skill).copied().map_or(0, |l| l.unwrap_or(1)),
                        skill.get_max_level().unwrap_or(1)
                    ))
                    .label_y(conrod_core::position::Relative::Scalar(tweak!(-28.0)))
                    .label_x(conrod_core::position::Relative::Scalar(tweak!(32.0)))
                    .label_color(TEXT_COLOR)
                    .label_font_size(self.fonts.cyri.scale(tweak!(16)))
                    .label_font_id(self.fonts.cyri.conrod_id)
                    .with_tooltip(
                        self.tooltip_manager,
                        "Double Strike Combo",
                        "Unlocks a second strike",
                        &diary_tooltip,
                        TEXT_COLOR,
                    )
                    .set(state.skill_axe_combo_0, ui)
                    .was_clicked()
                {
                    events.push(Event::UnlockSkill(skill));
                };
                let skill = Skill::Axe(DsDamage);
                if Button::image(self.imgs.axespin)
                    .w_h(tweak!(74.0), tweak!(74.0))
                    .middle_of(state.skills_top_l[1])
                    .label(&format!(
                        "{}/{}",
                        skills.get(&skill).copied().map_or(0, |l| l.unwrap_or(1)),
                        skill.get_max_level().unwrap_or(1)
                    ))
                    .label_y(conrod_core::position::Relative::Scalar(tweak!(-28.0)))
                    .label_x(conrod_core::position::Relative::Scalar(tweak!(32.0)))
                    .label_color(TEXT_COLOR)
                    .label_font_size(self.fonts.cyri.scale(tweak!(16)))
                    .label_font_id(self.fonts.cyri.conrod_id)
                    .with_tooltip(
                        self.tooltip_manager,
                        "Double Strike Damage",
                        "Increases damage scaling in combo",
                        &diary_tooltip,
                        TEXT_COLOR,
                    )
                    .set(state.skill_axe_combo_1, ui)
                    .was_clicked()
                {
                    events.push(Event::UnlockSkill(skill));
                };
                let skill = Skill::Axe(DsSpeed);
                if Button::image(self.imgs.axespin)
                    .w_h(tweak!(74.0), tweak!(74.0))
                    .middle_of(state.skills_top_l[2])
                    .label(&format!(
                        "{}/{}",
                        skills.get(&skill).copied().map_or(0, |l| l.unwrap_or(1)),
                        skill.get_max_level().unwrap_or(1)
                    ))
                    .label_y(conrod_core::position::Relative::Scalar(tweak!(-28.0)))
                    .label_x(conrod_core::position::Relative::Scalar(tweak!(32.0)))
                    .label_color(TEXT_COLOR)
                    .label_font_size(self.fonts.cyri.scale(tweak!(16)))
                    .label_font_id(self.fonts.cyri.conrod_id)
                    .with_tooltip(
                        self.tooltip_manager,
                        "Double Strike Speed",
                        "Increases speed scaling in combo",
                        &diary_tooltip,
                        TEXT_COLOR,
                    )
                    .set(state.skill_axe_combo_2, ui)
                    .was_clicked()
                {
                    events.push(Event::UnlockSkill(skill));
                };
                let skill = Skill::Axe(DsRegen);
                if Button::image(self.imgs.axespin)
                    .w_h(tweak!(74.0), tweak!(74.0))
                    .middle_of(state.skills_top_l[3])
                    .label(&format!(
                        "{}/{}",
                        skills.get(&skill).copied().map_or(0, |l| l.unwrap_or(1)),
                        skill.get_max_level().unwrap_or(1)
                    ))
                    .label_y(conrod_core::position::Relative::Scalar(tweak!(-28.0)))
                    .label_x(conrod_core::position::Relative::Scalar(tweak!(32.0)))
                    .label_color(TEXT_COLOR)
                    .label_font_size(self.fonts.cyri.scale(tweak!(16)))
                    .label_font_id(self.fonts.cyri.conrod_id)
                    .with_tooltip(
                        self.tooltip_manager,
                        "Double Strike Regen",
                        "Increases energy regen scaling in combo",
                        &diary_tooltip,
                        TEXT_COLOR,
                    )
                    .set(state.skill_axe_combo_3, ui)
                    .was_clicked()
                {
                    events.push(Event::UnlockSkill(skill));
                };
                // Top right skills
                let skill = Skill::Axe(SInfinite);
                if Button::image(self.imgs.axespin)
                    .w_h(tweak!(74.0), tweak!(74.0))
                    .middle_of(state.skills_top_r[0])
                    .label(&format!(
                        "{}/{}",
                        skills.get(&skill).copied().map_or(0, |l| l.unwrap_or(1)),
                        skill.get_max_level().unwrap_or(1)
                    ))
                    .label_y(conrod_core::position::Relative::Scalar(tweak!(-28.0)))
                    .label_x(conrod_core::position::Relative::Scalar(tweak!(32.0)))
                    .label_color(TEXT_COLOR)
                    .label_font_size(self.fonts.cyri.scale(tweak!(16)))
                    .label_font_id(self.fonts.cyri.conrod_id)
                    .with_tooltip(
                        self.tooltip_manager,
                        "Infinite Axe Spin",
                        "Spin for as long as you have energy",
                        &diary_tooltip,
                        TEXT_COLOR,
                    )
                    .set(state.skill_axe_spin_0, ui)
                    .was_clicked()
                {
                    events.push(Event::UnlockSkill(skill));
                };
                let skill = Skill::Axe(SDamage);
                if Button::image(self.imgs.axespin)
                    .w_h(tweak!(74.0), tweak!(74.0))
                    .middle_of(state.skills_top_r[1])
                    .label(&format!(
                        "{}/{}",
                        skills.get(&skill).copied().map_or(0, |l| l.unwrap_or(1)),
                        skill.get_max_level().unwrap_or(1)
                    ))
                    .label_y(conrod_core::position::Relative::Scalar(tweak!(-28.0)))
                    .label_x(conrod_core::position::Relative::Scalar(tweak!(32.0)))
                    .label_color(TEXT_COLOR)
                    .label_font_size(self.fonts.cyri.scale(tweak!(16)))
                    .label_font_id(self.fonts.cyri.conrod_id)
                    .with_tooltip(
                        self.tooltip_manager,
                        "Spin Damage",
                        "Increases the daamge each spin does",
                        &diary_tooltip,
                        TEXT_COLOR,
                    )
                    .set(state.skill_axe_spin_1, ui)
                    .was_clicked()
                {
                    events.push(Event::UnlockSkill(skill));
                };
                let skill = Skill::Axe(SHelicopter);
                if Button::image(self.imgs.axespin)
                    .w_h(tweak!(74.0), tweak!(74.0))
                    .middle_of(state.skills_top_r[2])
                    .label(&format!(
                        "{}/{}",
                        skills.get(&skill).copied().map_or(0, |l| l.unwrap_or(1)),
                        skill.get_max_level().unwrap_or(1)
                    ))
                    .label_y(conrod_core::position::Relative::Scalar(tweak!(-28.0)))
                    .label_x(conrod_core::position::Relative::Scalar(tweak!(32.0)))
                    .label_color(TEXT_COLOR)
                    .label_font_size(self.fonts.cyri.scale(tweak!(16)))
                    .label_font_id(self.fonts.cyri.conrod_id)
                    .with_tooltip(
                        self.tooltip_manager,
                        "Spin Helicopter",
                        "You fall a little slower while spinning",
                        &diary_tooltip,
                        TEXT_COLOR,
                    )
                    .set(state.skill_axe_spin_2, ui)
                    .was_clicked()
                {
                    events.push(Event::UnlockSkill(skill));
                };
                let skill = Skill::Axe(SSpeed);
                if Button::image(self.imgs.axespin)
                    .w_h(tweak!(74.0), tweak!(74.0))
                    .middle_of(state.skills_top_r[3])
                    .label(&format!(
                        "{}/{}",
                        skills.get(&skill).copied().map_or(0, |l| l.unwrap_or(1)),
                        skill.get_max_level().unwrap_or(1)
                    ))
                    .label_y(conrod_core::position::Relative::Scalar(tweak!(-28.0)))
                    .label_x(conrod_core::position::Relative::Scalar(tweak!(32.0)))
                    .label_color(TEXT_COLOR)
                    .label_font_size(self.fonts.cyri.scale(tweak!(16)))
                    .label_font_id(self.fonts.cyri.conrod_id)
                    .with_tooltip(
                        self.tooltip_manager,
                        "Spin Speed",
                        "Increases your spins per minute",
                        &diary_tooltip,
                        TEXT_COLOR,
                    )
                    .set(state.skill_axe_spin_3, ui)
                    .was_clicked()
                {
                    events.push(Event::UnlockSkill(skill));
                };
                let skill = Skill::Axe(SCost);
                if Button::image(self.imgs.axespin)
                    .w_h(tweak!(74.0), tweak!(74.0))
                    .middle_of(state.skills_top_r[4])
                    .label(&format!(
                        "{}/{}",
                        skills.get(&skill).copied().map_or(0, |l| l.unwrap_or(1)),
                        skill.get_max_level().unwrap_or(1)
                    ))
                    .label_y(conrod_core::position::Relative::Scalar(tweak!(-28.0)))
                    .label_x(conrod_core::position::Relative::Scalar(tweak!(32.0)))
                    .label_color(TEXT_COLOR)
                    .label_font_size(self.fonts.cyri.scale(tweak!(16)))
                    .label_font_id(self.fonts.cyri.conrod_id)
                    .with_tooltip(
                        self.tooltip_manager,
                        "Spin Cost",
                        "Increases your spin per energy efficiency",
                        &diary_tooltip,
                        TEXT_COLOR,
                    )
                    .set(state.skill_axe_spin_4, ui)
                    .was_clicked()
                {
                    events.push(Event::UnlockSkill(skill));
                };
                // Bottom left skills
                let skill = Skill::Axe(LUnlockLeap);
                if Button::image(self.imgs.axespin)
                    .w_h(tweak!(74.0), tweak!(74.0))
                    .middle_of(state.skills_bot_l[0])
                    .label(&format!(
                        "{}/{}",
                        skills.get(&skill).copied().map_or(0, |l| l.unwrap_or(1)),
                        skill.get_max_level().unwrap_or(1)
                    ))
                    .label_y(conrod_core::position::Relative::Scalar(tweak!(-28.0)))
                    .label_x(conrod_core::position::Relative::Scalar(tweak!(32.0)))
                    .label_color(TEXT_COLOR)
                    .label_font_size(self.fonts.cyri.scale(tweak!(16)))
                    .label_font_id(self.fonts.cyri.conrod_id)
                    .with_tooltip(
                        self.tooltip_manager,
                        "Unlock Leap",
                        "Unlocks a leap spin",
                        &diary_tooltip,
                        TEXT_COLOR,
                    )
                    .set(state.skill_axe_leap_0, ui)
                    .was_clicked()
                {
                    events.push(Event::UnlockSkill(skill));
                };
                let skill = Skill::Axe(LDamage);
                if Button::image(self.imgs.axespin)
                    .w_h(tweak!(74.0), tweak!(74.0))
                    .middle_of(state.skills_bot_l[1])
                    .label(&format!(
                        "{}/{}",
                        skills.get(&skill).copied().map_or(0, |l| l.unwrap_or(1)),
                        skill.get_max_level().unwrap_or(1)
                    ))
                    .label_y(conrod_core::position::Relative::Scalar(tweak!(-28.0)))
                    .label_x(conrod_core::position::Relative::Scalar(tweak!(32.0)))
                    .label_color(TEXT_COLOR)
                    .label_font_size(self.fonts.cyri.scale(tweak!(16)))
                    .label_font_id(self.fonts.cyri.conrod_id)
                    .with_tooltip(
                        self.tooltip_manager,
                        "Leap Damage",
                        "Increases damage of leap",
                        &diary_tooltip,
                        TEXT_COLOR,
                    )
                    .set(state.skill_axe_leap_1, ui)
                    .was_clicked()
                {
                    events.push(Event::UnlockSkill(skill));
                };
                let skill = Skill::Axe(LKnockback);
                if Button::image(self.imgs.axespin)
                    .w_h(tweak!(74.0), tweak!(74.0))
                    .middle_of(state.skills_bot_l[2])
                    .label(&format!(
                        "{}/{}",
                        skills.get(&skill).copied().map_or(0, |l| l.unwrap_or(1)),
                        skill.get_max_level().unwrap_or(1)
                    ))
                    .label_y(conrod_core::position::Relative::Scalar(tweak!(-28.0)))
                    .label_x(conrod_core::position::Relative::Scalar(tweak!(32.0)))
                    .label_color(TEXT_COLOR)
                    .label_font_size(self.fonts.cyri.scale(tweak!(16)))
                    .label_font_id(self.fonts.cyri.conrod_id)
                    .with_tooltip(
                        self.tooltip_manager,
                        "Leap Knockback",
                        "Increases knockback from leap",
                        &diary_tooltip,
                        TEXT_COLOR,
                    )
                    .set(state.skill_axe_leap_2, ui)
                    .was_clicked()
                {
                    events.push(Event::UnlockSkill(skill));
                };
                let skill = Skill::Axe(LCost);
                if Button::image(self.imgs.axespin)
                    .w_h(tweak!(74.0), tweak!(74.0))
                    .middle_of(state.skills_bot_l[3])
                    .label(&format!(
                        "{}/{}",
                        skills.get(&skill).copied().map_or(0, |l| l.unwrap_or(1)),
                        skill.get_max_level().unwrap_or(1)
                    ))
                    .label_y(conrod_core::position::Relative::Scalar(tweak!(-28.0)))
                    .label_x(conrod_core::position::Relative::Scalar(tweak!(32.0)))
                    .label_color(TEXT_COLOR)
                    .label_font_size(self.fonts.cyri.scale(tweak!(16)))
                    .label_font_id(self.fonts.cyri.conrod_id)
                    .with_tooltip(
                        self.tooltip_manager,
                        "Leap Cost",
                        "Decreases cost of leap",
                        &diary_tooltip,
                        TEXT_COLOR,
                    )
                    .set(state.skill_axe_leap_3, ui)
                    .was_clicked()
                {
                    events.push(Event::UnlockSkill(skill));
                };
                let skill = Skill::Axe(LDistance);
                if Button::image(self.imgs.axespin)
                    .w_h(tweak!(74.0), tweak!(74.0))
                    .middle_of(state.skills_bot_l[4])
                    .label(&format!(
                        "{}/{}",
                        skills.get(&skill).copied().map_or(0, |l| l.unwrap_or(1)),
                        skill.get_max_level().unwrap_or(1)
                    ))
                    .label_y(conrod_core::position::Relative::Scalar(tweak!(-28.0)))
                    .label_x(conrod_core::position::Relative::Scalar(tweak!(32.0)))
                    .label_color(TEXT_COLOR)
                    .label_font_size(self.fonts.cyri.scale(tweak!(16)))
                    .label_font_id(self.fonts.cyri.conrod_id)
                    .with_tooltip(
                        self.tooltip_manager,
                        "Leap Distance",
                        "Increases distance of leap",
                        &diary_tooltip,
                        TEXT_COLOR,
                    )
                    .set(state.skill_axe_leap_4, ui)
                    .was_clicked()
                {
                    events.push(Event::UnlockSkill(skill));
                };
            },
            SelectedSkillTree::Weapon(ToolKind::Hammer) => {
                use skills::HammerSkill::*;
                // Hammer
                Image::new(
                    self.item_imgs
                        .img_id_or_not_found_img(Tool("example_hammer".to_string())),
                )
                .wh(art_size)
                .middle_of(state.content_align)
                .color(Some(Color::Rgba(1.0, 1.0, 1.0, tweak!(1.0))))
                .set(state.hammer_render, ui);
                // Top Left skills
                //        5 1 6
                //        3 0 4
                //        8 2 7
                let skill = Skill::Hammer(SsKnockback);
                if Button::image(self.imgs.hammergolf)
                    .w_h(tweak!(74.0), tweak!(74.0))
                    .middle_of(state.skills_top_l[0])
                    .label(&format!(
                        "{}/{}",
                        skills.get(&skill).copied().map_or(0, |l| l.unwrap_or(1)),
                        skill.get_max_level().unwrap_or(1)
                    ))
                    .label_y(conrod_core::position::Relative::Scalar(tweak!(-28.0)))
                    .label_x(conrod_core::position::Relative::Scalar(tweak!(32.0)))
                    .label_color(TEXT_COLOR)
                    .label_font_size(self.fonts.cyri.scale(tweak!(16)))
                    .label_font_id(self.fonts.cyri.conrod_id)
                    .with_tooltip(
                        self.tooltip_manager,
                        "Single Strike Knockback",
                        "Increaes yeet potential of swings",
                        &diary_tooltip,
                        TEXT_COLOR,
                    )
                    .set(state.skill_hammer_combo_0, ui)
                    .was_clicked()
                {
                    events.push(Event::UnlockSkill(skill));
                };
                let skill = Skill::Hammer(SsDamage);
                if Button::image(self.imgs.hammergolf)
                    .w_h(tweak!(74.0), tweak!(74.0))
                    .middle_of(state.skills_top_l[1])
                    .label(&format!(
                        "{}/{}",
                        skills.get(&skill).copied().map_or(0, |l| l.unwrap_or(1)),
                        skill.get_max_level().unwrap_or(1)
                    ))
                    .label_y(conrod_core::position::Relative::Scalar(tweak!(-28.0)))
                    .label_x(conrod_core::position::Relative::Scalar(tweak!(32.0)))
                    .label_color(TEXT_COLOR)
                    .label_font_size(self.fonts.cyri.scale(tweak!(16)))
                    .label_font_id(self.fonts.cyri.conrod_id)
                    .with_tooltip(
                        self.tooltip_manager,
                        "Single Strike Damage",
                        "Increases damage scaling in combo",
                        &diary_tooltip,
                        TEXT_COLOR,
                    )
                    .set(state.skill_hammer_combo_1, ui)
                    .was_clicked()
                {
                    events.push(Event::UnlockSkill(skill));
                };
                let skill = Skill::Hammer(SsSpeed);
                if Button::image(self.imgs.hammergolf)
                    .w_h(tweak!(74.0), tweak!(74.0))
                    .middle_of(state.skills_top_l[2])
                    .label(&format!(
                        "{}/{}",
                        skills.get(&skill).copied().map_or(0, |l| l.unwrap_or(1)),
                        skill.get_max_level().unwrap_or(1)
                    ))
                    .label_y(conrod_core::position::Relative::Scalar(tweak!(-28.0)))
                    .label_x(conrod_core::position::Relative::Scalar(tweak!(32.0)))
                    .label_color(TEXT_COLOR)
                    .label_font_size(self.fonts.cyri.scale(tweak!(16)))
                    .label_font_id(self.fonts.cyri.conrod_id)
                    .with_tooltip(
                        self.tooltip_manager,
                        "Single Strike Speed",
                        "Increases speed scaling in combo",
                        &diary_tooltip,
                        TEXT_COLOR,
                    )
                    .set(state.skill_hammer_combo_2, ui)
                    .was_clicked()
                {
                    events.push(Event::UnlockSkill(skill));
                };
                let skill = Skill::Hammer(SsRegen);
                if Button::image(self.imgs.hammergolf)
                    .w_h(tweak!(74.0), tweak!(74.0))
                    .middle_of(state.skills_top_l[3])
                    .label(&format!(
                        "{}/{}",
                        skills.get(&skill).copied().map_or(0, |l| l.unwrap_or(1)),
                        skill.get_max_level().unwrap_or(1)
                    ))
                    .label_y(conrod_core::position::Relative::Scalar(tweak!(-28.0)))
                    .label_x(conrod_core::position::Relative::Scalar(tweak!(32.0)))
                    .label_color(TEXT_COLOR)
                    .label_font_size(self.fonts.cyri.scale(tweak!(16)))
                    .label_font_id(self.fonts.cyri.conrod_id)
                    .with_tooltip(
                        self.tooltip_manager,
                        "Single Strike Regen",
                        "Increases energy regen scaling in combo",
                        &diary_tooltip,
                        TEXT_COLOR,
                    )
                    .set(state.skill_hammer_combo_3, ui)
                    .was_clicked()
                {
                    events.push(Event::UnlockSkill(skill));
                };
                // Top right skills
                let skill = Skill::Hammer(CKnockback);
                if Button::image(self.imgs.hammergolf)
                    .w_h(tweak!(74.0), tweak!(74.0))
                    .middle_of(state.skills_top_r[0])
                    .label(&format!(
                        "{}/{}",
                        skills.get(&skill).copied().map_or(0, |l| l.unwrap_or(1)),
                        skill.get_max_level().unwrap_or(1)
                    ))
                    .label_y(conrod_core::position::Relative::Scalar(tweak!(-28.0)))
                    .label_x(conrod_core::position::Relative::Scalar(tweak!(32.0)))
                    .label_color(TEXT_COLOR)
                    .label_font_size(self.fonts.cyri.scale(tweak!(16)))
                    .label_font_id(self.fonts.cyri.conrod_id)
                    .with_tooltip(
                        self.tooltip_manager,
                        "Charged Melee Knockback",
                        "Massively increases yeet potential of swing",
                        &diary_tooltip,
                        TEXT_COLOR,
                    )
                    .set(state.skill_hammer_charged_0, ui)
                    .was_clicked()
                {
                    events.push(Event::UnlockSkill(skill));
                };
                let skill = Skill::Hammer(CDamage);
                if Button::image(self.imgs.hammergolf)
                    .w_h(tweak!(74.0), tweak!(74.0))
                    .middle_of(state.skills_top_r[1])
                    .label(&format!(
                        "{}/{}",
                        skills.get(&skill).copied().map_or(0, |l| l.unwrap_or(1)),
                        skill.get_max_level().unwrap_or(1)
                    ))
                    .label_y(conrod_core::position::Relative::Scalar(tweak!(-28.0)))
                    .label_x(conrod_core::position::Relative::Scalar(tweak!(32.0)))
                    .label_color(TEXT_COLOR)
                    .label_font_size(self.fonts.cyri.scale(tweak!(16)))
                    .label_font_id(self.fonts.cyri.conrod_id)
                    .with_tooltip(
                        self.tooltip_manager,
                        "Charged Melee Damage",
                        "Increases the daamge of the charged swing",
                        &diary_tooltip,
                        TEXT_COLOR,
                    )
                    .set(state.skill_hammer_charged_1, ui)
                    .was_clicked()
                {
                    events.push(Event::UnlockSkill(skill));
                };
                let skill = Skill::Hammer(CDrain);
                if Button::image(self.imgs.hammergolf)
                    .w_h(tweak!(74.0), tweak!(74.0))
                    .middle_of(state.skills_top_r[2])
                    .label(&format!(
                        "{}/{}",
                        skills.get(&skill).copied().map_or(0, |l| l.unwrap_or(1)),
                        skill.get_max_level().unwrap_or(1)
                    ))
                    .label_y(conrod_core::position::Relative::Scalar(tweak!(-28.0)))
                    .label_x(conrod_core::position::Relative::Scalar(tweak!(32.0)))
                    .label_color(TEXT_COLOR)
                    .label_font_size(self.fonts.cyri.scale(tweak!(16)))
                    .label_font_id(self.fonts.cyri.conrod_id)
                    .with_tooltip(
                        self.tooltip_manager,
                        "Charged Melee Energy Drain",
                        "Decreases the rate energy drains when charging",
                        &diary_tooltip,
                        TEXT_COLOR,
                    )
                    .set(state.skill_hammer_charged_2, ui)
                    .was_clicked()
                {
                    events.push(Event::UnlockSkill(skill));
                };
                let skill = Skill::Hammer(CSpeed);
                if Button::image(self.imgs.hammergolf)
                    .w_h(tweak!(74.0), tweak!(74.0))
                    .middle_of(state.skills_top_r[3])
                    .label(&format!(
                        "{}/{}",
                        skills.get(&skill).copied().map_or(0, |l| l.unwrap_or(1)),
                        skill.get_max_level().unwrap_or(1)
                    ))
                    .label_y(conrod_core::position::Relative::Scalar(tweak!(-28.0)))
                    .label_x(conrod_core::position::Relative::Scalar(tweak!(32.0)))
                    .label_color(TEXT_COLOR)
                    .label_font_size(self.fonts.cyri.scale(tweak!(16)))
                    .label_font_id(self.fonts.cyri.conrod_id)
                    .with_tooltip(
                        self.tooltip_manager,
                        "Charge Rate",
                        "Increases the rate that you charge the swing",
                        &diary_tooltip,
                        TEXT_COLOR,
                    )
                    .set(state.skill_hammer_charged_3, ui)
                    .was_clicked()
                {
                    events.push(Event::UnlockSkill(skill));
                };
                // Bottom left skills
                let skill = Skill::Hammer(LUnlockLeap);
                if Button::image(self.imgs.hammergolf)
                    .w_h(tweak!(74.0), tweak!(74.0))
                    .middle_of(state.skills_bot_l[0])
                    .label(&format!(
                        "{}/{}",
                        skills.get(&skill).copied().map_or(0, |l| l.unwrap_or(1)),
                        skill.get_max_level().unwrap_or(1)
                    ))
                    .label_y(conrod_core::position::Relative::Scalar(tweak!(-28.0)))
                    .label_x(conrod_core::position::Relative::Scalar(tweak!(32.0)))
                    .label_color(TEXT_COLOR)
                    .label_font_size(self.fonts.cyri.scale(tweak!(16)))
                    .label_font_id(self.fonts.cyri.conrod_id)
                    .with_tooltip(
                        self.tooltip_manager,
                        "Unlock Leap",
                        "Unlocks a leap",
                        &diary_tooltip,
                        TEXT_COLOR,
                    )
                    .set(state.skill_hammer_leap_0, ui)
                    .was_clicked()
                {
                    events.push(Event::UnlockSkill(skill));
                };
                let skill = Skill::Hammer(LDamage);
                if Button::image(self.imgs.hammergolf)
                    .w_h(tweak!(74.0), tweak!(74.0))
                    .middle_of(state.skills_bot_l[1])
                    .label(&format!(
                        "{}/{}",
                        skills.get(&skill).copied().map_or(0, |l| l.unwrap_or(1)),
                        skill.get_max_level().unwrap_or(1)
                    ))
                    .label_y(conrod_core::position::Relative::Scalar(tweak!(-28.0)))
                    .label_x(conrod_core::position::Relative::Scalar(tweak!(32.0)))
                    .label_color(TEXT_COLOR)
                    .label_font_size(self.fonts.cyri.scale(tweak!(16)))
                    .label_font_id(self.fonts.cyri.conrod_id)
                    .with_tooltip(
                        self.tooltip_manager,
                        "Leap Damage",
                        "Increases damage of leap",
                        &diary_tooltip,
                        TEXT_COLOR,
                    )
                    .set(state.skill_hammer_leap_1, ui)
                    .was_clicked()
                {
                    events.push(Event::UnlockSkill(skill));
                };
                let skill = Skill::Hammer(LKnockback);
                if Button::image(self.imgs.hammergolf)
                    .w_h(tweak!(74.0), tweak!(74.0))
                    .middle_of(state.skills_bot_l[2])
                    .label(&format!(
                        "{}/{}",
                        skills.get(&skill).copied().map_or(0, |l| l.unwrap_or(1)),
                        skill.get_max_level().unwrap_or(1)
                    ))
                    .label_y(conrod_core::position::Relative::Scalar(tweak!(-28.0)))
                    .label_x(conrod_core::position::Relative::Scalar(tweak!(32.0)))
                    .label_color(TEXT_COLOR)
                    .label_font_size(self.fonts.cyri.scale(tweak!(16)))
                    .label_font_id(self.fonts.cyri.conrod_id)
                    .with_tooltip(
                        self.tooltip_manager,
                        "Leap Knockback",
                        "Increases knockback from leap",
                        &diary_tooltip,
                        TEXT_COLOR,
                    )
                    .set(state.skill_hammer_leap_2, ui)
                    .was_clicked()
                {
                    events.push(Event::UnlockSkill(skill));
                };
                let skill = Skill::Hammer(LCost);
                if Button::image(self.imgs.hammergolf)
                    .w_h(tweak!(74.0), tweak!(74.0))
                    .middle_of(state.skills_bot_l[3])
                    .label(&format!(
                        "{}/{}",
                        skills.get(&skill).copied().map_or(0, |l| l.unwrap_or(1)),
                        skill.get_max_level().unwrap_or(1)
                    ))
                    .label_y(conrod_core::position::Relative::Scalar(tweak!(-28.0)))
                    .label_x(conrod_core::position::Relative::Scalar(tweak!(32.0)))
                    .label_color(TEXT_COLOR)
                    .label_font_size(self.fonts.cyri.scale(tweak!(16)))
                    .label_font_id(self.fonts.cyri.conrod_id)
                    .with_tooltip(
                        self.tooltip_manager,
                        "Leap Cost",
                        "Decreases cost of leap",
                        &diary_tooltip,
                        TEXT_COLOR,
                    )
                    .set(state.skill_hammer_leap_3, ui)
                    .was_clicked()
                {
                    events.push(Event::UnlockSkill(skill));
                };
                let skill = Skill::Hammer(LDistance);
                if Button::image(self.imgs.hammergolf)
                    .w_h(tweak!(74.0), tweak!(74.0))
                    .middle_of(state.skills_bot_l[4])
                    .label(&format!(
                        "{}/{}",
                        skills.get(&skill).copied().map_or(0, |l| l.unwrap_or(1)),
                        skill.get_max_level().unwrap_or(1)
                    ))
                    .label_y(conrod_core::position::Relative::Scalar(tweak!(-28.0)))
                    .label_x(conrod_core::position::Relative::Scalar(tweak!(32.0)))
                    .label_color(TEXT_COLOR)
                    .label_font_size(self.fonts.cyri.scale(tweak!(16)))
                    .label_font_id(self.fonts.cyri.conrod_id)
                    .with_tooltip(
                        self.tooltip_manager,
                        "Leap Distance",
                        "Increases distance of leap",
                        &diary_tooltip,
                        TEXT_COLOR,
                    )
                    .set(state.skill_hammer_leap_4, ui)
                    .was_clicked()
                {
                    events.push(Event::UnlockSkill(skill));
                };
                let skill = Skill::Hammer(LRange);
                if Button::image(self.imgs.hammergolf)
                    .w_h(tweak!(74.0), tweak!(74.0))
                    .middle_of(state.skills_bot_l[5])
                    .label(&format!(
                        "{}/{}",
                        skills.get(&skill).copied().map_or(0, |l| l.unwrap_or(1)),
                        skill.get_max_level().unwrap_or(1)
                    ))
                    .label_y(conrod_core::position::Relative::Scalar(tweak!(-28.0)))
                    .label_x(conrod_core::position::Relative::Scalar(tweak!(32.0)))
                    .label_color(TEXT_COLOR)
                    .label_font_size(self.fonts.cyri.scale(tweak!(16)))
                    .label_font_id(self.fonts.cyri.conrod_id)
                    .with_tooltip(
                        self.tooltip_manager,
                        "Leap Radius",
                        "Increases attack radius on ground slam",
                        &diary_tooltip,
                        TEXT_COLOR,
                    )
                    .set(state.skill_hammer_leap_5, ui)
                    .was_clicked()
                {
                    events.push(Event::UnlockSkill(skill));
                };
            },
            SelectedSkillTree::Weapon(ToolKind::Bow) => {
                use skills::BowSkill::*;
                // Bow
                Image::new(
                    self.item_imgs
                        .img_id_or_not_found_img(Tool("example_bow".to_string())),
                )
                .wh(art_size)
                .middle_of(state.content_align)
                .set(state.bow_render, ui);
                // Top Left skills
                //        5 1 6
                //        3 0 4
                //        8 2 7
                let skill = Skill::Bow(BDamage);
                if Button::image(self.imgs.bow_m1)
                    .w_h(tweak!(74.0), tweak!(74.0))
                    .middle_of(state.skills_top_l[0])
                    .label(&format!(
                        "{}/{}",
                        skills.get(&skill).copied().map_or(0, |l| l.unwrap_or(1)),
                        skill.get_max_level().unwrap_or(1)
                    ))
                    .label_y(conrod_core::position::Relative::Scalar(tweak!(-28.0)))
                    .label_x(conrod_core::position::Relative::Scalar(tweak!(32.0)))
                    .label_color(TEXT_COLOR)
                    .label_font_size(self.fonts.cyri.scale(tweak!(16)))
                    .label_font_id(self.fonts.cyri.conrod_id)
                    .with_tooltip(
                        self.tooltip_manager,
                        "Damage",
                        "Increases damage",
                        &diary_tooltip,
                        TEXT_COLOR,
                    )
                    .set(state.skill_bow_basic_0, ui)
                    .was_clicked()
                {
                    events.push(Event::UnlockSkill(skill));
                };
                let skill = Skill::Bow(BRegen);
                if Button::image(self.imgs.bow_m1)
                    .w_h(tweak!(74.0), tweak!(74.0))
                    .middle_of(state.skills_top_l[1])
                    .label(&format!(
                        "{}/{}",
                        skills.get(&skill).copied().map_or(0, |l| l.unwrap_or(1)),
                        skill.get_max_level().unwrap_or(1)
                    ))
                    .label_y(conrod_core::position::Relative::Scalar(tweak!(-28.0)))
                    .label_x(conrod_core::position::Relative::Scalar(tweak!(32.0)))
                    .label_color(TEXT_COLOR)
                    .label_font_size(self.fonts.cyri.scale(tweak!(16)))
                    .label_font_id(self.fonts.cyri.conrod_id)
                    .with_tooltip(
                        self.tooltip_manager,
                        "Energy Regen",
                        "Increases energy regen",
                        &diary_tooltip,
                        TEXT_COLOR,
                    )
                    .set(state.skill_bow_basic_1, ui)
                    .was_clicked()
                {
                    events.push(Event::UnlockSkill(skill));
                };
                // Top right skills
                let skill = Skill::Bow(CDamage);
                if Button::image(self.imgs.bow_m1)
                    .w_h(tweak!(74.0), tweak!(74.0))
                    .middle_of(state.skills_top_r[0])
                    .label(&format!(
                        "{}/{}",
                        skills.get(&skill).copied().map_or(0, |l| l.unwrap_or(1)),
                        skill.get_max_level().unwrap_or(1)
                    ))
                    .label_y(conrod_core::position::Relative::Scalar(tweak!(-28.0)))
                    .label_x(conrod_core::position::Relative::Scalar(tweak!(32.0)))
                    .label_color(TEXT_COLOR)
                    .label_font_size(self.fonts.cyri.scale(tweak!(16)))
                    .label_font_id(self.fonts.cyri.conrod_id)
                    .with_tooltip(
                        self.tooltip_manager,
                        "Charged Damage",
                        "Increases how much damage scales by as it is charged",
                        &diary_tooltip,
                        TEXT_COLOR,
                    )
                    .set(state.skill_bow_charged_0, ui)
                    .was_clicked()
                {
                    events.push(Event::UnlockSkill(skill));
                };
                let skill = Skill::Bow(CDrain);
                if Button::image(self.imgs.bow_m1)
                    .w_h(tweak!(74.0), tweak!(74.0))
                    .middle_of(state.skills_top_r[1])
                    .label(&format!(
                        "{}/{}",
                        skills.get(&skill).copied().map_or(0, |l| l.unwrap_or(1)),
                        skill.get_max_level().unwrap_or(1)
                    ))
                    .label_y(conrod_core::position::Relative::Scalar(tweak!(-28.0)))
                    .label_x(conrod_core::position::Relative::Scalar(tweak!(32.0)))
                    .label_color(TEXT_COLOR)
                    .label_font_size(self.fonts.cyri.scale(tweak!(16)))
                    .label_font_id(self.fonts.cyri.conrod_id)
                    .with_tooltip(
                        self.tooltip_manager,
                        "Charged Drain",
                        "Decreases the rate energy is drained while charging",
                        &diary_tooltip,
                        TEXT_COLOR,
                    )
                    .set(state.skill_bow_charged_1, ui)
                    .was_clicked()
                {
                    events.push(Event::UnlockSkill(skill));
                };
                let skill = Skill::Bow(CProjSpeed);
                if Button::image(self.imgs.bow_m1)
                    .w_h(tweak!(74.0), tweak!(74.0))
                    .middle_of(state.skills_top_r[2])
                    .label(&format!(
                        "{}/{}",
                        skills.get(&skill).copied().map_or(0, |l| l.unwrap_or(1)),
                        skill.get_max_level().unwrap_or(1)
                    ))
                    .label_y(conrod_core::position::Relative::Scalar(tweak!(-28.0)))
                    .label_x(conrod_core::position::Relative::Scalar(tweak!(32.0)))
                    .label_color(TEXT_COLOR)
                    .label_font_size(self.fonts.cyri.scale(tweak!(16)))
                    .label_font_id(self.fonts.cyri.conrod_id)
                    .with_tooltip(
                        self.tooltip_manager,
                        "Charged Projectile Speed",
                        "Increases yeet potential applied to arrow while charging",
                        &diary_tooltip,
                        TEXT_COLOR,
                    )
                    .set(state.skill_bow_charged_2, ui)
                    .was_clicked()
                {
                    events.push(Event::UnlockSkill(skill));
                };
                let skill = Skill::Bow(CSpeed);
                if Button::image(self.imgs.bow_m1)
                    .w_h(tweak!(74.0), tweak!(74.0))
                    .middle_of(state.skills_top_r[3])
                    .label(&format!(
                        "{}/{}",
                        skills.get(&skill).copied().map_or(0, |l| l.unwrap_or(1)),
                        skill.get_max_level().unwrap_or(1)
                    ))
                    .label_y(conrod_core::position::Relative::Scalar(tweak!(-28.0)))
                    .label_x(conrod_core::position::Relative::Scalar(tweak!(32.0)))
                    .label_color(TEXT_COLOR)
                    .label_font_size(self.fonts.cyri.scale(tweak!(16)))
                    .label_font_id(self.fonts.cyri.conrod_id)
                    .with_tooltip(
                        self.tooltip_manager,
                        "Charged Speed",
                        "Increases the rate that you charge the attack",
                        &diary_tooltip,
                        TEXT_COLOR,
                    )
                    .set(state.skill_bow_charged_3, ui)
                    .was_clicked()
                {
                    events.push(Event::UnlockSkill(skill));
                };
                let skill = Skill::Bow(CMove);
                if Button::image(self.imgs.bow_m1)
                    .w_h(tweak!(74.0), tweak!(74.0))
                    .middle_of(state.skills_top_r[4])
                    .label(&format!(
                        "{}/{}",
                        skills.get(&skill).copied().map_or(0, |l| l.unwrap_or(1)),
                        skill.get_max_level().unwrap_or(1)
                    ))
                    .label_y(conrod_core::position::Relative::Scalar(tweak!(-28.0)))
                    .label_x(conrod_core::position::Relative::Scalar(tweak!(32.0)))
                    .label_color(TEXT_COLOR)
                    .label_font_size(self.fonts.cyri.scale(tweak!(16)))
                    .label_font_id(self.fonts.cyri.conrod_id)
                    .with_tooltip(
                        self.tooltip_manager,
                        "Charged Move Speed",
                        "Increases how fast you can shuffle while charging the attack",
                        &diary_tooltip,
                        TEXT_COLOR,
                    )
                    .set(state.skill_bow_charged_4, ui)
                    .was_clicked()
                {
                    events.push(Event::UnlockSkill(skill));
                };
                let skill = Skill::Bow(CKnockback);
                if Button::image(self.imgs.bow_m1)
                    .w_h(tweak!(74.0), tweak!(74.0))
                    .middle_of(state.skills_top_r[5])
                    .label(&format!(
                        "{}/{}",
                        skills.get(&skill).copied().map_or(0, |l| l.unwrap_or(1)),
                        skill.get_max_level().unwrap_or(1)
                    ))
                    .label_y(conrod_core::position::Relative::Scalar(tweak!(-28.0)))
                    .label_x(conrod_core::position::Relative::Scalar(tweak!(32.0)))
                    .label_color(TEXT_COLOR)
                    .label_font_size(self.fonts.cyri.scale(tweak!(16)))
                    .label_font_id(self.fonts.cyri.conrod_id)
                    .with_tooltip(
                        self.tooltip_manager,
                        "Charged Knockback",
                        "Yeet enemies further",
                        &diary_tooltip,
                        TEXT_COLOR,
                    )
                    .set(state.skill_bow_charged_5, ui)
                    .was_clicked()
                {
                    events.push(Event::UnlockSkill(skill));
                };
                // Bottom left skills
                let skill = Skill::Bow(UnlockRepeater);
                if Button::image(self.imgs.bow_m1)
                    .w_h(tweak!(74.0), tweak!(74.0))
                    .middle_of(state.skills_bot_l[0])
                    .label(&format!(
                        "{}/{}",
                        skills.get(&skill).copied().map_or(0, |l| l.unwrap_or(1)),
                        skill.get_max_level().unwrap_or(1)
                    ))
                    .label_y(conrod_core::position::Relative::Scalar(tweak!(-28.0)))
                    .label_x(conrod_core::position::Relative::Scalar(tweak!(32.0)))
                    .label_color(TEXT_COLOR)
                    .label_font_size(self.fonts.cyri.scale(tweak!(16)))
                    .label_font_id(self.fonts.cyri.conrod_id)
                    .with_tooltip(
                        self.tooltip_manager,
                        "Repeater Unlock",
                        "Unlocks the ability to leap in the arrow",
                        &diary_tooltip,
                        TEXT_COLOR,
                    )
                    .set(state.skill_bow_repeater_0, ui)
                    .was_clicked()
                {
                    events.push(Event::UnlockSkill(skill));
                };
                let skill = Skill::Bow(RDamage);
                if Button::image(self.imgs.bow_m1)
                    .w_h(tweak!(74.0), tweak!(74.0))
                    .middle_of(state.skills_bot_l[1])
                    .label(&format!(
                        "{}/{}",
                        skills.get(&skill).copied().map_or(0, |l| l.unwrap_or(1)),
                        skill.get_max_level().unwrap_or(1)
                    ))
                    .label_y(conrod_core::position::Relative::Scalar(tweak!(-28.0)))
                    .label_x(conrod_core::position::Relative::Scalar(tweak!(32.0)))
                    .label_color(TEXT_COLOR)
                    .label_font_size(self.fonts.cyri.scale(tweak!(16)))
                    .label_font_id(self.fonts.cyri.conrod_id)
                    .with_tooltip(
                        self.tooltip_manager,
                        "Repeater Damage",
                        "Increases the damage done",
                        &diary_tooltip,
                        TEXT_COLOR,
                    )
                    .set(state.skill_bow_repeater_1, ui)
                    .was_clicked()
                {
                    events.push(Event::UnlockSkill(skill));
                };
                let skill = Skill::Bow(RGlide);
                if Button::image(self.imgs.bow_m1)
                    .w_h(tweak!(74.0), tweak!(74.0))
                    .middle_of(state.skills_bot_l[2])
                    .label(&format!(
                        "{}/{}",
                        skills.get(&skill).copied().map_or(0, |l| l.unwrap_or(1)),
                        skill.get_max_level().unwrap_or(1)
                    ))
                    .label_y(conrod_core::position::Relative::Scalar(tweak!(-28.0)))
                    .label_x(conrod_core::position::Relative::Scalar(tweak!(32.0)))
                    .label_color(TEXT_COLOR)
                    .label_font_size(self.fonts.cyri.scale(tweak!(16)))
                    .label_font_id(self.fonts.cyri.conrod_id)
                    .with_tooltip(
                        self.tooltip_manager,
                        "Repeater Glide",
                        "Glide further while repeatering",
                        &diary_tooltip,
                        TEXT_COLOR,
                    )
                    .set(state.skill_bow_repeater_2, ui)
                    .was_clicked()
                {
                    events.push(Event::UnlockSkill(skill));
                };
                let skill = Skill::Bow(RCost);
                if Button::image(self.imgs.bow_m1)
                    .w_h(tweak!(74.0), tweak!(74.0))
                    .middle_of(state.skills_bot_l[3])
                    .label(&format!(
                        "{}/{}",
                        skills.get(&skill).copied().map_or(0, |l| l.unwrap_or(1)),
                        skill.get_max_level().unwrap_or(1)
                    ))
                    .label_y(conrod_core::position::Relative::Scalar(tweak!(-28.0)))
                    .label_x(conrod_core::position::Relative::Scalar(tweak!(32.0)))
                    .label_color(TEXT_COLOR)
                    .label_font_size(self.fonts.cyri.scale(tweak!(16)))
                    .label_font_id(self.fonts.cyri.conrod_id)
                    .with_tooltip(
                        self.tooltip_manager,
                        "Repeater Cost",
                        "Decreases the energy cost to become a gliding repeater",
                        &diary_tooltip,
                        TEXT_COLOR,
                    )
                    .set(state.skill_bow_repeater_3, ui)
                    .was_clicked()
                {
                    events.push(Event::UnlockSkill(skill));
                };
                let skill = Skill::Bow(RArrows);
                if Button::image(self.imgs.bow_m1)
                    .w_h(tweak!(74.0), tweak!(74.0))
                    .middle_of(state.skills_bot_l[4])
                    .label(&format!(
                        "{}/{}",
                        skills.get(&skill).copied().map_or(0, |l| l.unwrap_or(1)),
                        skill.get_max_level().unwrap_or(1)
                    ))
                    .label_y(conrod_core::position::Relative::Scalar(tweak!(-28.0)))
                    .label_x(conrod_core::position::Relative::Scalar(tweak!(32.0)))
                    .label_color(TEXT_COLOR)
                    .label_font_size(self.fonts.cyri.scale(tweak!(16)))
                    .label_font_id(self.fonts.cyri.conrod_id)
                    .with_tooltip(
                        self.tooltip_manager,
                        "Arrow Count",
                        "Yeet more arrows when you leap",
                        &diary_tooltip,
                        TEXT_COLOR,
                    )
                    .set(state.skill_bow_repeater_4, ui)
                    .was_clicked()
                {
                    events.push(Event::UnlockSkill(skill));
                };
                // Bottom right skills
                let skill = Skill::Bow(ProjSpeed);
                if Button::image(self.imgs.bow_m1)
                    .w_h(tweak!(74.0), tweak!(74.0))
                    .middle_of(state.skills_bot_r[0])
                    .label(&format!(
                        "{}/{}",
                        skills.get(&skill).copied().map_or(0, |l| l.unwrap_or(1)),
                        skill.get_max_level().unwrap_or(1)
                    ))
                    .label_y(conrod_core::position::Relative::Scalar(tweak!(-28.0)))
                    .label_x(conrod_core::position::Relative::Scalar(tweak!(32.0)))
                    .label_color(TEXT_COLOR)
                    .label_font_size(self.fonts.cyri.scale(tweak!(16)))
                    .label_font_id(self.fonts.cyri.conrod_id)
                    .with_tooltip(
                        self.tooltip_manager,
                        "Projectile Speed",
                        "Allows you to yeet arrows further, faster",
                        &diary_tooltip,
                        TEXT_COLOR,
                    )
                    .set(state.skill_bow_passive_0, ui)
                    .was_clicked()
                {
                    events.push(Event::UnlockSkill(skill));
                };
            },
            SelectedSkillTree::Weapon(ToolKind::Staff) => {
                use skills::StaffSkill::*;
                // Staff
                Image::new(
                    self.item_imgs
                        .img_id_or_not_found_img(Tool("example_staff_fire".to_string())),
                )
                .wh(art_size)
                .middle_of(state.content_align)
                .color(Some(Color::Rgba(1.0, 1.0, 1.0, tweak!(1.0))))
                .set(state.staff_render, ui);
                // Top Left skills
                //        5 1 6
                //        3 0 4
                //        8 2 7
                let skill = Skill::Staff(BExplosion);
                if Button::image(self.imgs.fireball)
                    .w_h(tweak!(74.0), tweak!(74.0))
                    .middle_of(state.skills_top_l[0])
                    .label(&format!(
                        "{}/{}",
                        skills.get(&skill).copied().map_or(0, |l| l.unwrap_or(1)),
                        skill.get_max_level().unwrap_or(1)
                    ))
                    .label_y(conrod_core::position::Relative::Scalar(tweak!(-28.0)))
                    .label_x(conrod_core::position::Relative::Scalar(tweak!(32.0)))
                    .label_color(TEXT_COLOR)
                    .label_font_size(self.fonts.cyri.scale(tweak!(16)))
                    .label_font_id(self.fonts.cyri.conrod_id)
                    .with_tooltip(
                        self.tooltip_manager,
                        "Explosion",
                        "When fire just isn't enough",
                        &diary_tooltip,
                        TEXT_COLOR,
                    )
                    .set(state.skill_staff_basic_0, ui)
                    .was_clicked()
                {
                    events.push(Event::UnlockSkill(skill));
                };
                let skill = Skill::Staff(BDamage);
                if Button::image(self.imgs.fireball)
                    .w_h(tweak!(74.0), tweak!(74.0))
                    .middle_of(state.skills_top_l[1])
                    .label(&format!(
                        "{}/{}",
                        skills.get(&skill).copied().map_or(0, |l| l.unwrap_or(1)),
                        skill.get_max_level().unwrap_or(1)
                    ))
                    .label_y(conrod_core::position::Relative::Scalar(tweak!(-28.0)))
                    .label_x(conrod_core::position::Relative::Scalar(tweak!(32.0)))
                    .label_color(TEXT_COLOR)
                    .label_font_size(self.fonts.cyri.scale(tweak!(16)))
                    .label_font_id(self.fonts.cyri.conrod_id)
                    .with_tooltip(
                        self.tooltip_manager,
                        "Damage",
                        "Increases damage",
                        &diary_tooltip,
                        TEXT_COLOR,
                    )
                    .set(state.skill_staff_basic_1, ui)
                    .was_clicked()
                {
                    events.push(Event::UnlockSkill(skill));
                };
                let skill = Skill::Staff(BRegen);
                if Button::image(self.imgs.fireball)
                    .w_h(tweak!(74.0), tweak!(74.0))
                    .middle_of(state.skills_top_l[2])
                    .label(&format!(
                        "{}/{}",
                        skills.get(&skill).copied().map_or(0, |l| l.unwrap_or(1)),
                        skill.get_max_level().unwrap_or(1)
                    ))
                    .label_y(conrod_core::position::Relative::Scalar(tweak!(-28.0)))
                    .label_x(conrod_core::position::Relative::Scalar(tweak!(32.0)))
                    .label_color(TEXT_COLOR)
                    .label_font_size(self.fonts.cyri.scale(tweak!(16)))
                    .label_font_id(self.fonts.cyri.conrod_id)
                    .with_tooltip(
                        self.tooltip_manager,
                        "Energy Regen",
                        "Increases energy regen",
                        &diary_tooltip,
                        TEXT_COLOR,
                    )
                    .set(state.skill_staff_basic_2, ui)
                    .was_clicked()
                {
                    events.push(Event::UnlockSkill(skill));
                };
                let skill = Skill::Staff(BRadius);
                if Button::image(self.imgs.fireball)
                    .w_h(tweak!(74.0), tweak!(74.0))
                    .middle_of(state.skills_top_l[3])
                    .label(&format!(
                        "{}/{}",
                        skills.get(&skill).copied().map_or(0, |l| l.unwrap_or(1)),
                        skill.get_max_level().unwrap_or(1)
                    ))
                    .label_y(conrod_core::position::Relative::Scalar(tweak!(-28.0)))
                    .label_x(conrod_core::position::Relative::Scalar(tweak!(32.0)))
                    .label_color(TEXT_COLOR)
                    .label_font_size(self.fonts.cyri.scale(tweak!(16)))
                    .label_font_id(self.fonts.cyri.conrod_id)
                    .with_tooltip(
                        self.tooltip_manager,
                        "Explosion Radius",
                        "Bigger is better",
                        &diary_tooltip,
                        TEXT_COLOR,
                    )
                    .set(state.skill_staff_basic_3, ui)
                    .was_clicked()
                {
                    events.push(Event::UnlockSkill(skill));
                };
                // Top right skills
                let skill = Skill::Staff(FDamage);
                if Button::image(self.imgs.fireball)
                    .w_h(tweak!(74.0), tweak!(74.0))
                    .middle_of(state.skills_top_r[0])
                    .label(&format!(
                        "{}/{}",
                        skills.get(&skill).copied().map_or(0, |l| l.unwrap_or(1)),
                        skill.get_max_level().unwrap_or(1)
                    ))
                    .label_y(conrod_core::position::Relative::Scalar(tweak!(-28.0)))
                    .label_x(conrod_core::position::Relative::Scalar(tweak!(32.0)))
                    .label_color(TEXT_COLOR)
                    .label_font_size(self.fonts.cyri.scale(tweak!(16)))
                    .label_font_id(self.fonts.cyri.conrod_id)
                    .with_tooltip(
                        self.tooltip_manager,
                        "Flamethrower Damage",
                        "Increases damage",
                        &diary_tooltip,
                        TEXT_COLOR,
                    )
                    .set(state.skill_staff_beam_0, ui)
                    .was_clicked()
                {
                    events.push(Event::UnlockSkill(skill));
                };
                let skill = Skill::Staff(FDrain);
                if Button::image(self.imgs.fireball)
                    .w_h(tweak!(74.0), tweak!(74.0))
                    .middle_of(state.skills_top_r[1])
                    .label(&format!(
                        "{}/{}",
                        skills.get(&skill).copied().map_or(0, |l| l.unwrap_or(1)),
                        skill.get_max_level().unwrap_or(1)
                    ))
                    .label_y(conrod_core::position::Relative::Scalar(tweak!(-28.0)))
                    .label_x(conrod_core::position::Relative::Scalar(tweak!(32.0)))
                    .label_color(TEXT_COLOR)
                    .label_font_size(self.fonts.cyri.scale(tweak!(16)))
                    .label_font_id(self.fonts.cyri.conrod_id)
                    .with_tooltip(
                        self.tooltip_manager,
                        "Energy Drain",
                        "Decreases the rate energy is drained",
                        &diary_tooltip,
                        TEXT_COLOR,
                    )
                    .set(state.skill_staff_beam_1, ui)
                    .was_clicked()
                {
                    events.push(Event::UnlockSkill(skill));
                };
                let skill = Skill::Staff(FRange);
                if Button::image(self.imgs.fireball)
                    .w_h(tweak!(74.0), tweak!(74.0))
                    .middle_of(state.skills_top_r[2])
                    .label(&format!(
                        "{}/{}",
                        skills.get(&skill).copied().map_or(0, |l| l.unwrap_or(1)),
                        skill.get_max_level().unwrap_or(1)
                    ))
                    .label_y(conrod_core::position::Relative::Scalar(tweak!(-28.0)))
                    .label_x(conrod_core::position::Relative::Scalar(tweak!(32.0)))
                    .label_color(TEXT_COLOR)
                    .label_font_size(self.fonts.cyri.scale(tweak!(16)))
                    .label_font_id(self.fonts.cyri.conrod_id)
                    .with_tooltip(
                        self.tooltip_manager,
                        "Flamethrower Range",
                        "For when the flames just won't reach",
                        &diary_tooltip,
                        TEXT_COLOR,
                    )
                    .set(state.skill_staff_beam_2, ui)
                    .was_clicked()
                {
                    events.push(Event::UnlockSkill(skill));
                };
                let skill = Skill::Staff(FVelocity);
                if Button::image(self.imgs.fireball)
                    .w_h(tweak!(74.0), tweak!(74.0))
                    .middle_of(state.skills_top_r[3])
                    .label(&format!(
                        "{}/{}",
                        skills.get(&skill).copied().map_or(0, |l| l.unwrap_or(1)),
                        skill.get_max_level().unwrap_or(1)
                    ))
                    .label_y(conrod_core::position::Relative::Scalar(tweak!(-28.0)))
                    .label_x(conrod_core::position::Relative::Scalar(tweak!(32.0)))
                    .label_color(TEXT_COLOR)
                    .label_font_size(self.fonts.cyri.scale(tweak!(16)))
                    .label_font_id(self.fonts.cyri.conrod_id)
                    .with_tooltip(
                        self.tooltip_manager,
                        "Flame Velocity",
                        "Gets the fire there faster",
                        &diary_tooltip,
                        TEXT_COLOR,
                    )
                    .set(state.skill_staff_beam_3, ui)
                    .was_clicked()
                {
                    events.push(Event::UnlockSkill(skill));
                };
                // Bottom left skills
                let skill = Skill::Staff(UnlockShockwave);
                if Button::image(self.imgs.fireball)
                    .w_h(tweak!(74.0), tweak!(74.0))
                    .middle_of(state.skills_bot_l[0])
                    .label(&format!(
                        "{}/{}",
                        skills.get(&skill).copied().map_or(0, |l| l.unwrap_or(1)),
                        skill.get_max_level().unwrap_or(1)
                    ))
                    .label_y(conrod_core::position::Relative::Scalar(tweak!(-28.0)))
                    .label_x(conrod_core::position::Relative::Scalar(tweak!(32.0)))
                    .label_color(TEXT_COLOR)
                    .label_font_size(self.fonts.cyri.scale(tweak!(16)))
                    .label_font_id(self.fonts.cyri.conrod_id)
                    .with_tooltip(
                        self.tooltip_manager,
                        "Shockwave Unlock",
                        "Unlocks the ability to yeet enemies away using fire",
                        &diary_tooltip,
                        TEXT_COLOR,
                    )
                    .set(state.skill_staff_shockwave_0, ui)
                    .was_clicked()
                {
                    events.push(Event::UnlockSkill(skill));
                };
                let skill = Skill::Staff(SDamage);
                if Button::image(self.imgs.fireball)
                    .w_h(tweak!(74.0), tweak!(74.0))
                    .middle_of(state.skills_bot_l[1])
                    .label(&format!(
                        "{}/{}",
                        skills.get(&skill).copied().map_or(0, |l| l.unwrap_or(1)),
                        skill.get_max_level().unwrap_or(1)
                    ))
                    .label_y(conrod_core::position::Relative::Scalar(tweak!(-28.0)))
                    .label_x(conrod_core::position::Relative::Scalar(tweak!(32.0)))
                    .label_color(TEXT_COLOR)
                    .label_font_size(self.fonts.cyri.scale(tweak!(16)))
                    .label_font_id(self.fonts.cyri.conrod_id)
                    .with_tooltip(
                        self.tooltip_manager,
                        "Shockwave Damage",
                        "Increases the damage done",
                        &diary_tooltip,
                        TEXT_COLOR,
                    )
                    .set(state.skill_staff_shockwave_1, ui)
                    .was_clicked()
                {
                    events.push(Event::UnlockSkill(skill));
                };
                let skill = Skill::Staff(SKnockback);
                if Button::image(self.imgs.fireball)
                    .w_h(tweak!(74.0), tweak!(74.0))
                    .middle_of(state.skills_bot_l[2])
                    .label(&format!(
                        "{}/{}",
                        skills.get(&skill).copied().map_or(0, |l| l.unwrap_or(1)),
                        skill.get_max_level().unwrap_or(1)
                    ))
                    .label_y(conrod_core::position::Relative::Scalar(tweak!(-28.0)))
                    .label_x(conrod_core::position::Relative::Scalar(tweak!(32.0)))
                    .label_color(TEXT_COLOR)
                    .label_font_size(self.fonts.cyri.scale(tweak!(16)))
                    .label_font_id(self.fonts.cyri.conrod_id)
                    .with_tooltip(
                        self.tooltip_manager,
                        "Shockwave Knockback",
                        "Increases yeet potential",
                        &diary_tooltip,
                        TEXT_COLOR,
                    )
                    .set(state.skill_staff_shockwave_2, ui)
                    .was_clicked()
                {
                    events.push(Event::UnlockSkill(skill));
                };
                let skill = Skill::Staff(SCost);
                if Button::image(self.imgs.fireball)
                    .w_h(tweak!(74.0), tweak!(74.0))
                    .middle_of(state.skills_bot_l[3])
                    .label(&format!(
                        "{}/{}",
                        skills.get(&skill).copied().map_or(0, |l| l.unwrap_or(1)),
                        skill.get_max_level().unwrap_or(1)
                    ))
                    .label_y(conrod_core::position::Relative::Scalar(tweak!(-28.0)))
                    .label_x(conrod_core::position::Relative::Scalar(tweak!(32.0)))
                    .label_color(TEXT_COLOR)
                    .label_font_size(self.fonts.cyri.scale(tweak!(16)))
                    .label_font_id(self.fonts.cyri.conrod_id)
                    .with_tooltip(
                        self.tooltip_manager,
                        "Shockwave Cost",
                        "Decreases the energy cost to yeet helpless villagers",
                        &diary_tooltip,
                        TEXT_COLOR,
                    )
                    .set(state.skill_staff_shockwave_3, ui)
                    .was_clicked()
                {
                    events.push(Event::UnlockSkill(skill));
                };
                let skill = Skill::Staff(SRange);
                if Button::image(self.imgs.fireball)
                    .w_h(tweak!(74.0), tweak!(74.0))
                    .middle_of(state.skills_bot_l[4])
                    .label(&format!(
                        "{}/{}",
                        skills.get(&skill).copied().map_or(0, |l| l.unwrap_or(1)),
                        skill.get_max_level().unwrap_or(1)
                    ))
                    .label_y(conrod_core::position::Relative::Scalar(tweak!(-28.0)))
                    .label_x(conrod_core::position::Relative::Scalar(tweak!(32.0)))
                    .label_color(TEXT_COLOR)
                    .label_font_size(self.fonts.cyri.scale(tweak!(16)))
                    .label_font_id(self.fonts.cyri.conrod_id)
                    .with_tooltip(
                        self.tooltip_manager,
                        "Shockwave Range",
                        "Yeet things that used to be out of reach",
                        &diary_tooltip,
                        TEXT_COLOR,
                    )
                    .set(state.skill_staff_shockwave_4, ui)
                    .was_clicked()
                {
                    events.push(Event::UnlockSkill(skill));
                };
            },
            SelectedSkillTree::Weapon(ToolKind::Sceptre) => {
                use skills::SceptreSkill::*;
                // Sceptre
                Image::new(
                    self.item_imgs
                        .img_id_or_not_found_img(Tool("example_sceptre".to_string())),
                )
                .wh(art_size)
                .middle_of(state.content_align)
                .color(Some(Color::Rgba(1.0, 1.0, 1.0, tweak!(1.0))))
                .set(state.sceptre_render, ui);
                // Top Left skills
                //        5 1 6
                //        3 0 4
                //        8 2 7
                let skill = Skill::Sceptre(BHeal);
                if Button::image(self.imgs.heal_0)
                    .w_h(tweak!(74.0), tweak!(74.0))
                    .middle_of(state.skills_top_l[0])
                    .label(&format!(
                        "{}/{}",
                        skills.get(&skill).copied().map_or(0, |l| l.unwrap_or(1)),
                        skill.get_max_level().unwrap_or(1)
                    ))
                    .label_y(conrod_core::position::Relative::Scalar(tweak!(-28.0)))
                    .label_x(conrod_core::position::Relative::Scalar(tweak!(32.0)))
                    .label_color(TEXT_COLOR)
                    .label_font_size(self.fonts.cyri.scale(tweak!(16)))
                    .label_font_id(self.fonts.cyri.conrod_id)
                    .with_tooltip(
                        self.tooltip_manager,
                        "Beam Heal",
                        "Increased healing from the beam",
                        &diary_tooltip,
                        TEXT_COLOR,
                    )
                    .set(state.skill_sceptre_beam_0, ui)
                    .was_clicked()
                {
                    events.push(Event::UnlockSkill(skill));
                };
                let skill = Skill::Sceptre(BDamage);
                if Button::image(self.imgs.heal_0)
                    .w_h(tweak!(74.0), tweak!(74.0))
                    .middle_of(state.skills_top_l[1])
                    .label(&format!(
                        "{}/{}",
                        skills.get(&skill).copied().map_or(0, |l| l.unwrap_or(1)),
                        skill.get_max_level().unwrap_or(1)
                    ))
                    .label_y(conrod_core::position::Relative::Scalar(tweak!(-28.0)))
                    .label_x(conrod_core::position::Relative::Scalar(tweak!(32.0)))
                    .label_color(TEXT_COLOR)
                    .label_font_size(self.fonts.cyri.scale(tweak!(16)))
                    .label_font_id(self.fonts.cyri.conrod_id)
                    .with_tooltip(
                        self.tooltip_manager,
                        "Damage",
                        "Increases damage",
                        &diary_tooltip,
                        TEXT_COLOR,
                    )
                    .set(state.skill_sceptre_beam_1, ui)
                    .was_clicked()
                {
                    events.push(Event::UnlockSkill(skill));
                };
                let skill = Skill::Sceptre(BRegen);
                if Button::image(self.imgs.heal_0)
                    .w_h(tweak!(74.0), tweak!(74.0))
                    .middle_of(state.skills_top_l[2])
                    .label(&format!(
                        "{}/{}",
                        skills.get(&skill).copied().map_or(0, |l| l.unwrap_or(1)),
                        skill.get_max_level().unwrap_or(1)
                    ))
                    .label_y(conrod_core::position::Relative::Scalar(tweak!(-28.0)))
                    .label_x(conrod_core::position::Relative::Scalar(tweak!(32.0)))
                    .label_color(TEXT_COLOR)
                    .label_font_size(self.fonts.cyri.scale(tweak!(16)))
                    .label_font_id(self.fonts.cyri.conrod_id)
                    .with_tooltip(
                        self.tooltip_manager,
                        "Energy Regen",
                        "Increases energy regen from damage",
                        &diary_tooltip,
                        TEXT_COLOR,
                    )
                    .set(state.skill_sceptre_beam_2, ui)
                    .was_clicked()
                {
                    events.push(Event::UnlockSkill(skill));
                };
                let skill = Skill::Sceptre(BRange);
                if Button::image(self.imgs.heal_0)
                    .w_h(tweak!(74.0), tweak!(74.0))
                    .middle_of(state.skills_top_l[3])
                    .label(&format!(
                        "{}/{}",
                        skills.get(&skill).copied().map_or(0, |l| l.unwrap_or(1)),
                        skill.get_max_level().unwrap_or(1)
                    ))
                    .label_y(conrod_core::position::Relative::Scalar(tweak!(-28.0)))
                    .label_x(conrod_core::position::Relative::Scalar(tweak!(32.0)))
                    .label_color(TEXT_COLOR)
                    .label_font_size(self.fonts.cyri.scale(tweak!(16)))
                    .label_font_id(self.fonts.cyri.conrod_id)
                    .with_tooltip(
                        self.tooltip_manager,
                        "Range",
                        "Longer beam",
                        &diary_tooltip,
                        TEXT_COLOR,
                    )
                    .set(state.skill_sceptre_beam_3, ui)
                    .was_clicked()
                {
                    events.push(Event::UnlockSkill(skill));
                };
                let skill = Skill::Sceptre(BLifesteal);
                if Button::image(self.imgs.heal_0)
                    .w_h(tweak!(74.0), tweak!(74.0))
                    .middle_of(state.skills_top_l[4])
                    .label(&format!(
                        "{}/{}",
                        skills.get(&skill).copied().map_or(0, |l| l.unwrap_or(1)),
                        skill.get_max_level().unwrap_or(1)
                    ))
                    .label_y(conrod_core::position::Relative::Scalar(tweak!(-28.0)))
                    .label_x(conrod_core::position::Relative::Scalar(tweak!(32.0)))
                    .label_color(TEXT_COLOR)
                    .label_font_size(self.fonts.cyri.scale(tweak!(16)))
                    .label_font_id(self.fonts.cyri.conrod_id)
                    .with_tooltip(
                        self.tooltip_manager,
                        "Lifesteal Efficiency",
                        "Thieve more health",
                        &diary_tooltip,
                        TEXT_COLOR,
                    )
                    .set(state.skill_sceptre_beam_4, ui)
                    .was_clicked()
                {
                    events.push(Event::UnlockSkill(skill));
                };
                let skill = Skill::Sceptre(BCost);
                if Button::image(self.imgs.heal_0)
                    .w_h(tweak!(74.0), tweak!(74.0))
                    .middle_of(state.skills_top_l[5])
                    .label(&format!(
                        "{}/{}",
                        skills.get(&skill).copied().map_or(0, |l| l.unwrap_or(1)),
                        skill.get_max_level().unwrap_or(1)
                    ))
                    .label_y(conrod_core::position::Relative::Scalar(tweak!(-28.0)))
                    .label_x(conrod_core::position::Relative::Scalar(tweak!(32.0)))
                    .label_color(TEXT_COLOR)
                    .label_font_size(self.fonts.cyri.scale(tweak!(16)))
                    .label_font_id(self.fonts.cyri.conrod_id)
                    .with_tooltip(
                        self.tooltip_manager,
                        "Heal Cost",
                        "Use less energy when healing",
                        &diary_tooltip,
                        TEXT_COLOR,
                    )
                    .set(state.skill_sceptre_beam_5, ui)
                    .was_clicked()
                {
                    events.push(Event::UnlockSkill(skill));
                };
                // Top right skills
                let skill = Skill::Sceptre(PHeal);
                if Button::image(self.imgs.heal_0)
                    .w_h(tweak!(74.0), tweak!(74.0))
                    .middle_of(state.skills_top_r[0])
                    .label(&format!(
                        "{}/{}",
                        skills.get(&skill).copied().map_or(0, |l| l.unwrap_or(1)),
                        skill.get_max_level().unwrap_or(1)
                    ))
                    .label_y(conrod_core::position::Relative::Scalar(tweak!(-28.0)))
                    .label_x(conrod_core::position::Relative::Scalar(tweak!(32.0)))
                    .label_color(TEXT_COLOR)
                    .label_font_size(self.fonts.cyri.scale(tweak!(16)))
                    .label_font_id(self.fonts.cyri.conrod_id)
                    .with_tooltip(
                        self.tooltip_manager,
                        "Heal",
                        "Increases healing",
                        &diary_tooltip,
                        TEXT_COLOR,
                    )
                    .set(state.skill_sceptre_bomb_0, ui)
                    .was_clicked()
                {
                    events.push(Event::UnlockSkill(skill));
                };
                let skill = Skill::Sceptre(PDamage);
                if Button::image(self.imgs.heal_0)
                    .w_h(tweak!(74.0), tweak!(74.0))
                    .middle_of(state.skills_top_r[1])
                    .label(&format!(
                        "{}/{}",
                        skills.get(&skill).copied().map_or(0, |l| l.unwrap_or(1)),
                        skill.get_max_level().unwrap_or(1)
                    ))
                    .label_y(conrod_core::position::Relative::Scalar(tweak!(-28.0)))
                    .label_x(conrod_core::position::Relative::Scalar(tweak!(32.0)))
                    .label_color(TEXT_COLOR)
                    .label_font_size(self.fonts.cyri.scale(tweak!(16)))
                    .label_font_id(self.fonts.cyri.conrod_id)
                    .with_tooltip(
                        self.tooltip_manager,
                        "Damage",
                        "Increases damage",
                        &diary_tooltip,
                        TEXT_COLOR,
                    )
                    .set(state.skill_sceptre_bomb_1, ui)
                    .was_clicked()
                {
                    events.push(Event::UnlockSkill(skill));
                };
                let skill = Skill::Sceptre(PRadius);
                if Button::image(self.imgs.heal_0)
                    .w_h(tweak!(74.0), tweak!(74.0))
                    .middle_of(state.skills_top_r[2])
                    .label(&format!(
                        "{}/{}",
                        skills.get(&skill).copied().map_or(0, |l| l.unwrap_or(1)),
                        skill.get_max_level().unwrap_or(1)
                    ))
                    .label_y(conrod_core::position::Relative::Scalar(tweak!(-28.0)))
                    .label_x(conrod_core::position::Relative::Scalar(tweak!(32.0)))
                    .label_color(TEXT_COLOR)
                    .label_font_size(self.fonts.cyri.scale(tweak!(16)))
                    .label_font_id(self.fonts.cyri.conrod_id)
                    .with_tooltip(
                        self.tooltip_manager,
                        "Radius",
                        "Increases radius",
                        &diary_tooltip,
                        TEXT_COLOR,
                    )
                    .set(state.skill_sceptre_bomb_2, ui)
                    .was_clicked()
                {
                    events.push(Event::UnlockSkill(skill));
                };
                let skill = Skill::Sceptre(PCost);
                if Button::image(self.imgs.heal_0)
                    .w_h(tweak!(74.0), tweak!(74.0))
                    .middle_of(state.skills_top_r[3])
                    .label(&format!(
                        "{}/{}",
                        skills.get(&skill).copied().map_or(0, |l| l.unwrap_or(1)),
                        skill.get_max_level().unwrap_or(1)
                    ))
                    .label_y(conrod_core::position::Relative::Scalar(tweak!(-28.0)))
                    .label_x(conrod_core::position::Relative::Scalar(tweak!(32.0)))
                    .label_color(TEXT_COLOR)
                    .label_font_size(self.fonts.cyri.scale(tweak!(16)))
                    .label_font_id(self.fonts.cyri.conrod_id)
                    .with_tooltip(
                        self.tooltip_manager,
                        "Energy Cost",
                        "Decreases energy cost of bomb",
                        &diary_tooltip,
                        TEXT_COLOR,
                    )
                    .set(state.skill_sceptre_bomb_3, ui)
                    .was_clicked()
                {
                    events.push(Event::UnlockSkill(skill));
                };
                let skill = Skill::Sceptre(PProjSpeed);
                if Button::image(self.imgs.heal_0)
                    .w_h(tweak!(74.0), tweak!(74.0))
                    .middle_of(state.skills_top_r[4])
                    .label(&format!(
                        "{}/{}",
                        skills.get(&skill).copied().map_or(0, |l| l.unwrap_or(1)),
                        skill.get_max_level().unwrap_or(1)
                    ))
                    .label_y(conrod_core::position::Relative::Scalar(tweak!(-28.0)))
                    .label_x(conrod_core::position::Relative::Scalar(tweak!(32.0)))
                    .label_color(TEXT_COLOR)
                    .label_font_size(self.fonts.cyri.scale(tweak!(16)))
                    .label_font_id(self.fonts.cyri.conrod_id)
                    .with_tooltip(
                        self.tooltip_manager,
                        "Projectile Speed",
                        "Yeets it faster",
                        &diary_tooltip,
                        TEXT_COLOR,
                    )
                    .set(state.skill_sceptre_bomb_4, ui)
                    .was_clicked()
                {
                    events.push(Event::UnlockSkill(skill));
                };
            },
            _ => {},
        }

        events
    }
}

fn skill_tree_from_str(string: &str) -> Option<SelectedSkillTree> {
    match string {
        "General Combat" => Some(SelectedSkillTree::General),
        "Sword" => Some(SelectedSkillTree::Weapon(ToolKind::Sword)),
        "Hammer" => Some(SelectedSkillTree::Weapon(ToolKind::Hammer)),
        "Axe" => Some(SelectedSkillTree::Weapon(ToolKind::Axe)),
        "Sceptre" => Some(SelectedSkillTree::Weapon(ToolKind::Sceptre)),
        "Bow" => Some(SelectedSkillTree::Weapon(ToolKind::Bow)),
        "Fire Staff" => Some(SelectedSkillTree::Weapon(ToolKind::Staff)),
        _ => None,
    }
}
