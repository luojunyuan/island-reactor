use std::cell::RefCell;
use std::rc::Rc;

use rustc_hash::FxHashMap;
use windows_core::{IInspectable, Interface, Result};

use crate::bindings as Xaml;
use crate::core::backend::*;
use crate::core::*;
use muxc_bindings as Muxc;

/// Backend that creates native `Windows.UI.Xaml` controls for XAML Islands.
pub struct WinUIBackend {
    controls: RefCell<FxHashMap<ControlId, Handle>>,
    event_revokers: RefCell<FxHashMap<(ControlId, Event), Vec<EventSubscription>>>,
    templated_selection_revokers: RefCell<FxHashMap<ControlId, EventSubscription>>,
    templated_realizers: RefCell<FxHashMap<ControlId, Rc<dyn Fn(usize)>>>,
    parent_children: RefCell<FxHashMap<ControlId, Vec<ControlId>>>,
    templated_rows: RefCell<FxHashMap<ControlId, Vec<Option<ControlId>>>>,
    next_id: RefCell<u32>,
}

impl Default for WinUIBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl WinUIBackend {
    pub fn new() -> Self {
        Self {
            controls: RefCell::new(FxHashMap::default()),
            event_revokers: RefCell::new(FxHashMap::default()),
            templated_selection_revokers: RefCell::new(FxHashMap::default()),
            templated_realizers: RefCell::new(FxHashMap::default()),
            parent_children: RefCell::new(FxHashMap::default()),
            templated_rows: RefCell::new(FxHashMap::default()),
            next_id: RefCell::new(0),
        }
    }

    pub fn get_ui_element(&self, id: ControlId) -> Option<IInspectable> {
        self.controls
            .borrow()
            .get(&id)
            .and_then(|h| h.as_ui_element().ok())
            .and_then(|u| u.cast().ok())
    }

    fn alloc_id(&self) -> ControlId {
        let mut counter = self.next_id.borrow_mut();
        *counter += 1;
        ControlId::new(*counter)
    }

    fn visual_index(&self, parent: ControlId, logical: usize) -> usize {
        self.parent_children
            .borrow()
            .get(&parent)
            .map_or(logical, |children| logical.min(children.len()))
    }

    fn set_prop_inner(&self, id: ControlId, prop: Prop, value: &PropValue) -> Result<()> {
        let map = self.controls.borrow();
        let Some(handle) = map.get(&id) else {
            return Ok(());
        };

        match (prop, value, handle) {
            (Prop::Text, PropValue::Str(s), Handle::TextBlock(tb)) => tb.SetText(&hstring(s))?,
            (Prop::Text, PropValue::Str(s), Handle::TextBox(tb)) => {
                if tb.Text().ok().map(|text| text.to_string_lossy()) != Some(s.clone()) {
                    tb.SetText(&hstring(s))?;
                }
            }
            (
                Prop::TextWrapping | Prop::TextWrappingWrap,
                PropValue::I32(v),
                Handle::TextBlock(tb),
            ) => {
                tb.SetTextWrapping(Xaml::TextWrapping(*v))?;
            }
            (
                Prop::TextWrapping | Prop::TextWrappingWrap,
                PropValue::I32(v),
                Handle::TextBox(tb),
            ) => {
                tb.SetTextWrapping(Xaml::TextWrapping(*v))?;
            }
            (Prop::IsTextSelectionEnabled, PropValue::Bool(v), Handle::TextBlock(tb)) => {
                tb.SetIsTextSelectionEnabled(*v)?;
            }
            (Prop::PlaceholderText, PropValue::Str(s), h) => {
                if let Ok(tb) = h.cast_inner::<Xaml::TextBox>() {
                    tb.SetPlaceholderText(&hstring(s))?;
                } else if let Ok(pb) = h.cast_inner::<Xaml::PasswordBox>() {
                    pb.SetPlaceholderText(&hstring(s))?;
                } else if let Ok(asb) = h.cast_inner::<Xaml::AutoSuggestBox>() {
                    asb.SetPlaceholderText(&hstring(s))?;
                } else if let Ok(cdp) = h.cast_inner::<Xaml::CalendarDatePicker>() {
                    cdp.SetPlaceholderText(&hstring(s))?;
                }
            }
            (Prop::Header, PropValue::Str(s), h) => {
                let content = string_reference(s);
                if let Ok(tb) = h.cast_inner::<Xaml::TextBox>() {
                    tb.SetHeader(&content)?;
                } else if let Ok(pb) = h.cast_inner::<Xaml::PasswordBox>() {
                    pb.SetHeader(&content)?;
                } else if let Ok(cb) = h.cast_inner::<Xaml::ComboBox>() {
                    cb.SetHeader(&content)?;
                } else if let Ok(slider) = h.cast_inner::<Xaml::Slider>() {
                    slider.SetHeader(&content)?;
                } else if let Ok(tp) = h.cast_inner::<Xaml::TimePicker>() {
                    tp.SetHeader(&content)?;
                } else if let Ok(dp) = h.cast_inner::<Xaml::DatePicker>() {
                    dp.SetHeader(&content)?;
                } else if let Ok(cdp) = h.cast_inner::<Xaml::CalendarDatePicker>() {
                    cdp.SetHeader(&content)?;
                } else if let Ok(asb) = h.cast_inner::<Xaml::AutoSuggestBox>() {
                    asb.SetHeader(&content)?;
                } else if let Ok(reb) = h.cast_inner::<Xaml::RichEditBox>() {
                    reb.SetHeader(&content)?;
                } else if let Ok(expander) = h.cast_inner::<Muxc::Expander>() {
                    expander.SetHeader(&content)?;
                } else if let Ok(number) = h.cast_inner::<Muxc::NumberBox>() {
                    number.SetHeader(&content)?;
                } else if let Ok(radio) = h.cast_inner::<Muxc::RadioButtons>() {
                    radio.SetHeader(&content)?;
                } else if let Ok(tab) = h.cast_inner::<Muxc::TabViewItem>() {
                    tab.SetHeader(&content)?;
                }
            }
            (Prop::Content, PropValue::Str(s), h) => {
                set_content_string(h, s)?;
            }
            (Prop::IsEnabled, PropValue::Bool(v), h) => {
                if let Ok(control) = h.cast_inner::<Xaml::Control>() {
                    control.SetIsEnabled(*v)?;
                }
            }
            (Prop::IsChecked, PropValue::Bool(v), h) => {
                if let Ok(toggle) = h.cast_inner::<Xaml::ToggleButton>() {
                    toggle.SetIsChecked(Some(*v))?;
                }
            }
            (Prop::IsOn, PropValue::Bool(v), Handle::ToggleSwitch(ts)) => {
                ts.SetIsOn(*v)?;
            }
            (Prop::OnContent, PropValue::Str(s), Handle::ToggleSwitch(ts)) => {
                ts.SetOnContent(&string_reference(s))?;
            }
            (Prop::OffContent, PropValue::Str(s), Handle::ToggleSwitch(ts)) => {
                ts.SetOffContent(&string_reference(s))?;
            }
            (Prop::Orientation, PropValue::I32(v), Handle::StackPanel(sp)) => {
                sp.SetOrientation(Xaml::Orientation(*v))?;
            }
            (Prop::Orientation, PropValue::I32(v), Handle::Slider(slider)) => {
                slider.SetOrientation(Xaml::Orientation(*v))?;
            }
            (Prop::Spacing, PropValue::F64(v), Handle::StackPanel(sp)) => {
                sp.SetSpacing(*v)?;
            }
            (Prop::RowSpacing, PropValue::F64(v), Handle::Grid(g)) => {
                g.SetRowSpacing(*v)?;
            }
            (Prop::ColumnSpacing, PropValue::F64(v), Handle::Grid(g)) => {
                g.SetColumnSpacing(*v)?;
            }
            (Prop::Margin, PropValue::Thickness(t), h) => {
                if let Ok(fe) = h.cast_inner::<Xaml::FrameworkElement>() {
                    fe.SetMargin(xaml_thickness(*t))?;
                }
            }
            (Prop::Padding, PropValue::Thickness(t), h) => {
                if let Ok(control) = h.cast_inner::<Xaml::Control>() {
                    control.SetPadding(xaml_thickness(*t))?;
                } else if let Ok(border) = h.cast_inner::<Xaml::Border>() {
                    border.SetPadding(xaml_thickness(*t))?;
                }
            }
            (Prop::Width, PropValue::F64(v), h) => {
                if let Ok(fe) = h.cast_inner::<Xaml::FrameworkElement>() {
                    fe.SetWidth(*v)?;
                }
            }
            (Prop::Height, PropValue::F64(v), h) => {
                if let Ok(fe) = h.cast_inner::<Xaml::FrameworkElement>() {
                    fe.SetHeight(*v)?;
                }
            }
            (Prop::MinWidth, PropValue::F64(v), h) => {
                if let Ok(fe) = h.cast_inner::<Xaml::FrameworkElement>() {
                    fe.SetMinWidth(*v)?;
                }
            }
            (Prop::MaxWidth, PropValue::F64(v), h) => {
                if let Ok(fe) = h.cast_inner::<Xaml::FrameworkElement>() {
                    fe.SetMaxWidth(*v)?;
                }
            }
            (Prop::MinHeight, PropValue::F64(v), h) => {
                if let Ok(fe) = h.cast_inner::<Xaml::FrameworkElement>() {
                    fe.SetMinHeight(*v)?;
                }
            }
            (Prop::MaxHeight, PropValue::F64(v), h) => {
                if let Ok(fe) = h.cast_inner::<Xaml::FrameworkElement>() {
                    fe.SetMaxHeight(*v)?;
                }
            }
            (Prop::HorizontalAlignment, PropValue::I32(v), h) => {
                if let Ok(fe) = h.cast_inner::<Xaml::FrameworkElement>() {
                    fe.SetHorizontalAlignment(Xaml::HorizontalAlignment(*v))?;
                }
            }
            (Prop::VerticalAlignment, PropValue::I32(v), h) => {
                if let Ok(fe) = h.cast_inner::<Xaml::FrameworkElement>() {
                    fe.SetVerticalAlignment(Xaml::VerticalAlignment(*v))?;
                }
            }
            (Prop::Opacity, PropValue::F64(v), h) => {
                if let Ok(ui) = h.cast_inner::<Xaml::UIElement>() {
                    ui.SetOpacity(*v)?;
                }
            }
            (Prop::FontSize, PropValue::F64(v), h) => {
                if let Ok(control) = h.cast_inner::<Xaml::Control>() {
                    control.SetFontSize(*v)?;
                } else if let Ok(tb) = h.cast_inner::<Xaml::TextBlock>() {
                    tb.SetFontSize(*v)?;
                } else if let Ok(rtb) = h.cast_inner::<Xaml::RichTextBlock>() {
                    rtb.SetFontSize(*v)?;
                }
            }
            (Prop::FontWeight, PropValue::U16(v), h) => {
                let weight = Xaml::FontWeight { Weight: *v };
                if let Ok(control) = h.cast_inner::<Xaml::Control>() {
                    control.SetFontWeight(weight)?;
                } else if let Ok(tb) = h.cast_inner::<Xaml::TextBlock>() {
                    tb.SetFontWeight(weight)?;
                }
            }
            (Prop::FontFamily, PropValue::Str(s), h) => {
                let family = Xaml::FontFamily::CreateInstanceWithName(&hstring(s))?;
                if let Ok(control) = h.cast_inner::<Xaml::Control>() {
                    control.SetFontFamily(&family)?;
                } else if let Ok(tb) = h.cast_inner::<Xaml::TextBlock>() {
                    tb.SetFontFamily(&family)?;
                } else if let Ok(rtb) = h.cast_inner::<Xaml::RichTextBlock>() {
                    rtb.SetFontFamily(&family)?;
                }
            }
            (Prop::Foreground, PropValue::Brush(b), h) => {
                let brush = xaml_brush(b)?;
                if let Ok(control) = h.cast_inner::<Xaml::Control>() {
                    control.SetForeground(&brush)?;
                } else if let Ok(tb) = h.cast_inner::<Xaml::TextBlock>() {
                    tb.SetForeground(&brush)?;
                } else if let Ok(rtb) = h.cast_inner::<Xaml::RichTextBlock>() {
                    rtb.SetForeground(&brush)?;
                }
            }
            (Prop::Background, PropValue::Brush(b), h) => {
                let brush = xaml_brush(b)?;
                if let Ok(border) = h.cast_inner::<Xaml::Border>() {
                    border.SetBackground(&brush)?;
                } else if let Ok(panel) = h.cast_inner::<Xaml::Panel>() {
                    panel.SetBackground(&brush)?;
                } else if let Ok(control) = h.cast_inner::<Xaml::Control>() {
                    control.SetBackground(&brush)?;
                }
            }
            (Prop::StyleVariant, PropValue::I32(v), Handle::Button(button)) => {
                apply_button_style_variant(button, *v)?;
            }
            (Prop::BorderBrush, PropValue::Brush(b), Handle::Border(border)) => {
                border.SetBorderBrush(&xaml_brush(b)?)?;
            }
            (Prop::BorderThickness, PropValue::Thickness(t), Handle::Border(border)) => {
                border.SetBorderThickness(xaml_thickness(*t))?;
            }
            (Prop::CornerRadius, PropValue::F64(v), Handle::Border(border)) => {
                border.SetCornerRadius(Xaml::CornerRadius {
                    TopLeft: *v,
                    TopRight: *v,
                    BottomRight: *v,
                    BottomLeft: *v,
                })?;
            }
            (Prop::GridRows, PropValue::GridLengths(rows), Handle::Grid(grid)) => {
                set_grid_rows(grid, rows)?;
            }
            (Prop::GridColumns, PropValue::GridLengths(cols), Handle::Grid(grid)) => {
                set_grid_columns(grid, cols)?;
            }
            (Prop::AttachedGridRow, PropValue::I32(v), h) => {
                let element = h.as_framework_element()?;
                Xaml::Grid::SetRow(&element, *v)?;
            }
            (Prop::AttachedGridColumn, PropValue::I32(v), h) => {
                let element = h.as_framework_element()?;
                Xaml::Grid::SetColumn(&element, *v)?;
            }
            (Prop::AttachedGridRowSpan, PropValue::I32(v), h) => {
                let element = h.as_framework_element()?;
                Xaml::Grid::SetRowSpan(&element, *v)?;
            }
            (Prop::AttachedGridColumnSpan, PropValue::I32(v), h) => {
                let element = h.as_framework_element()?;
                Xaml::Grid::SetColumnSpan(&element, *v)?;
            }
            (Prop::AttachedCanvasLeft, PropValue::F64(v), h) => {
                let element = h.as_ui_element()?;
                Xaml::Canvas::SetLeft(&element, *v)?;
            }
            (Prop::AttachedCanvasTop, PropValue::F64(v), h) => {
                let element = h.as_ui_element()?;
                Xaml::Canvas::SetTop(&element, *v)?;
            }
            (Prop::AttachedCanvasZIndex, PropValue::I32(v), h) => {
                let element = h.as_ui_element()?;
                Xaml::Canvas::SetZIndex(&element, *v)?;
            }
            (Prop::AlignLeftWithPanel, PropValue::Bool(v), h) => {
                let element = h.as_ui_element()?;
                Xaml::RelativePanel::SetAlignLeftWithPanel(&element, *v)?;
            }
            (Prop::AlignRightWithPanel, PropValue::Bool(v), h) => {
                let element = h.as_ui_element()?;
                Xaml::RelativePanel::SetAlignRightWithPanel(&element, *v)?;
            }
            (Prop::AlignTopWithPanel, PropValue::Bool(v), h) => {
                let element = h.as_ui_element()?;
                Xaml::RelativePanel::SetAlignTopWithPanel(&element, *v)?;
            }
            (Prop::AlignBottomWithPanel, PropValue::Bool(v), h) => {
                let element = h.as_ui_element()?;
                Xaml::RelativePanel::SetAlignBottomWithPanel(&element, *v)?;
            }
            (Prop::AlignHCenterWithPanel, PropValue::Bool(v), h) => {
                let element = h.as_ui_element()?;
                Xaml::RelativePanel::SetAlignHorizontalCenterWithPanel(&element, *v)?;
            }
            (Prop::AlignVCenterWithPanel, PropValue::Bool(v), h) => {
                let element = h.as_ui_element()?;
                Xaml::RelativePanel::SetAlignVerticalCenterWithPanel(&element, *v)?;
            }
            (Prop::ImageSource, PropValue::Str(uri), Handle::Image(image)) => {
                set_image_uri(image, uri)?;
            }
            (Prop::Stretch, PropValue::I32(v), h) => {
                if let Ok(image) = h.cast_inner::<Xaml::Image>() {
                    image.SetStretch(Xaml::Stretch(*v))?;
                } else if let Ok(viewbox) = h.cast_inner::<Xaml::Viewbox>() {
                    viewbox.SetStretch(Xaml::Stretch(*v))?;
                }
            }
            (Prop::Value, PropValue::F64(v), h) => {
                if let Ok(range) = h.cast_inner::<Xaml::RangeBase>() {
                    range.SetValue2(*v)?;
                } else if let Ok(rating) = h.cast_inner::<Xaml::RatingControl>() {
                    rating.SetValue2(*v)?;
                } else if let Ok(number) = h.cast_inner::<Muxc::NumberBox>() {
                    number.SetValue2(*v)?;
                } else if let Ok(cp) = h.cast_inner::<Xaml::ColorPicker>() {
                    let _ = cp.SetColor(Xaml::Color {
                        A: 255,
                        R: *v as u8,
                        G: *v as u8,
                        B: *v as u8,
                    });
                }
            }
            (Prop::Value, PropValue::I32(v), Handle::InfoBadge(badge)) => {
                badge.SetValue2(*v)?;
            }
            (Prop::Minimum, PropValue::F64(v), h) => {
                if let Ok(range) = h.cast_inner::<Xaml::RangeBase>() {
                    range.SetMinimum(*v)?;
                } else if let Ok(number) = h.cast_inner::<Muxc::NumberBox>() {
                    number.SetMinimum(*v)?;
                }
            }
            (Prop::Maximum, PropValue::F64(v), h) => {
                if let Ok(range) = h.cast_inner::<Xaml::RangeBase>() {
                    range.SetMaximum(*v)?;
                } else if let Ok(number) = h.cast_inner::<Muxc::NumberBox>() {
                    number.SetMaximum(*v)?;
                }
            }
            (Prop::Step, PropValue::F64(v), Handle::Slider(slider)) => {
                slider.SetStepFrequency(*v)?;
            }
            (Prop::IsActive, PropValue::Bool(v), Handle::ProgressRing(ring)) => {
                ring.SetIsActive(*v)?;
            }
            (Prop::IsExpanded, PropValue::Bool(v), Handle::Expander(expander)) => {
                expander.SetIsExpanded(*v)?;
            }
            (Prop::IsIndeterminate, PropValue::Bool(v), h) => {
                if let Ok(bar) = h.cast_inner::<Xaml::ProgressBar>() {
                    bar.SetIsIndeterminate(*v)?;
                }
            }
            (Prop::SelectedIndex, PropValue::I32(v), h) => {
                if let Ok(selector) = h.cast_inner::<Xaml::Selector>() {
                    selector.SetSelectedIndex(*v)?;
                } else if let Ok(pivot) = h.cast_inner::<Xaml::Pivot>() {
                    pivot.SetSelectedIndex(*v)?;
                } else if let Ok(radio) = h.cast_inner::<Muxc::RadioButtons>() {
                    radio.SetSelectedIndex(*v)?;
                } else if let Ok(tab) = h.cast_inner::<Muxc::TabView>() {
                    tab.SetSelectedIndex(*v)?;
                }
            }
            (Prop::SelectionMode, PropValue::I32(v), h) => {
                if let Ok(list) = h.cast_inner::<Xaml::ListView>() {
                    list.SetSelectionMode(Xaml::ListViewSelectionMode(*v))?;
                } else if let Ok(grid) = h.cast_inner::<Xaml::GridView>() {
                    grid.SetSelectionMode(Xaml::ListViewSelectionMode(*v))?;
                } else if let Ok(tree) = h.cast_inner::<Xaml::TreeView>() {
                    tree.SetSelectionMode(Xaml::TreeViewSelectionMode(*v))?;
                }
            }
            (Prop::Items, PropValue::StrList(items), h) => {
                if let Ok(radio) = h.cast_inner::<Muxc::RadioButtons>() {
                    set_radio_buttons_items(&radio, items)?;
                } else if let Ok(breadcrumb) = h.cast_inner::<Muxc::BreadcrumbBar>() {
                    set_breadcrumb_items(&breadcrumb, items)?;
                } else {
                    set_string_items(h, items)?;
                }
            }
            (Prop::AutoSuggestItems, PropValue::StrList(items), Handle::AutoSuggestBox(asb)) => {
                set_string_items_for_items_control(asb, items)?;
            }
            (Prop::AutoSuggestPlaceholder, PropValue::Str(s), Handle::AutoSuggestBox(asb)) => {
                asb.SetPlaceholderText(&hstring(s))?;
            }
            (Prop::MenuItems, PropValue::NavMenuItems(items), Handle::NavigationView(nav)) => {
                set_nav_items(nav, items)?;
            }
            (Prop::SelectedTag, PropValue::Str(tag), Handle::NavigationView(nav)) => {
                select_nav_tag(nav, tag)?;
            }
            (Prop::PaneDisplayMode, PropValue::I32(v), Handle::NavigationView(nav)) => {
                nav.SetPaneDisplayMode(Muxc::NavigationViewPaneDisplayMode(*v))?;
            }
            (Prop::IsPaneOpen, PropValue::Bool(v), h) => {
                if let Ok(nav) = h.cast_inner::<Muxc::NavigationView>() {
                    nav.SetIsPaneOpen(*v)?;
                } else if let Ok(split) = h.cast_inner::<Xaml::SplitView>() {
                    split.SetIsPaneOpen(*v)?;
                }
            }
            (Prop::IsSettingsVisible, PropValue::Bool(v), Handle::NavigationView(nav)) => {
                nav.SetIsSettingsVisible(*v)?;
            }
            (Prop::IsPaneToggleButtonVisible, PropValue::Bool(v), Handle::NavigationView(nav)) => {
                nav.SetIsPaneToggleButtonVisible(*v)?;
            }
            (Prop::IsBackButtonVisible, PropValue::Bool(v), Handle::NavigationView(nav)) => {
                nav.SetIsBackButtonVisible(if *v {
                    Muxc::NavigationViewBackButtonVisible::Auto
                } else {
                    Muxc::NavigationViewBackButtonVisible::Collapsed
                })?;
            }
            (Prop::IsBackEnabled, PropValue::Bool(v), Handle::NavigationView(nav)) => {
                nav.SetIsBackEnabled(*v)?;
            }
            (Prop::PaneTitle, PropValue::Str(s), Handle::NavigationView(nav)) => {
                nav.SetPaneTitle(&hstring(s))?;
            }
            (Prop::Title, PropValue::Str(s), Handle::InfoBar(info)) => {
                info.SetTitle(&hstring(s))?;
            }
            (Prop::Message, PropValue::Str(s), Handle::InfoBar(info)) => {
                info.SetMessage(&hstring(s))?;
            }
            (Prop::Severity, PropValue::I32(v), Handle::InfoBar(info)) => {
                info.SetSeverity(Muxc::InfoBarSeverity(*v))?;
            }
            (Prop::IsOpen, PropValue::Bool(v), Handle::InfoBar(info)) => {
                info.SetIsOpen(*v)?;
            }
            (Prop::IsClosable, PropValue::Bool(v), Handle::InfoBar(info)) => {
                info.SetIsClosable(*v)?;
            }
            (Prop::Title, PropValue::Str(s), Handle::TeachingTip(tip)) => {
                tip.SetTitle(&hstring(s))?;
            }
            (Prop::Subtitle, PropValue::Str(s), Handle::TeachingTip(tip)) => {
                tip.SetSubtitle(&hstring(s))?;
            }
            (Prop::IsOpen, PropValue::Bool(v), Handle::TeachingTip(tip)) => {
                tip.SetIsOpen(*v)?;
            }
            (Prop::IsLightDismissEnabled, PropValue::Bool(v), Handle::TeachingTip(tip)) => {
                tip.SetIsLightDismissEnabled(*v)?;
            }
            (Prop::PreferredPlacement, PropValue::I32(v), Handle::TeachingTip(tip)) => {
                tip.SetPreferredPlacement(Muxc::TeachingTipPlacementMode(*v))?;
            }
            (Prop::ActionButtonText, PropValue::Str(s), Handle::TeachingTip(tip)) => {
                tip.SetActionButtonContent(&string_reference(s))?;
            }
            (Prop::CloseButtonText, PropValue::Str(s), Handle::TeachingTip(tip)) => {
                tip.SetCloseButtonContent(&string_reference(s))?;
            }
            (Prop::MaxColumns, PropValue::I32(v), Handle::RadioButtons(radio)) => {
                radio.SetMaxColumns(*v)?;
            }
            (Prop::CanReorderTabs, PropValue::Bool(v), Handle::TabView(tab)) => {
                tab.SetCanReorderTabs(*v)?;
            }
            (Prop::IsAddTabButtonVisible, PropValue::Bool(v), Handle::TabView(tab)) => {
                tab.SetIsAddTabButtonVisible(*v)?;
            }
            (Prop::ItemKey, PropValue::Str(s), Handle::TabViewItem(tab)) => {
                tab.cast::<Xaml::FrameworkElement>()?
                    .SetTag(&string_reference(s))?;
            }
            (Prop::IsClosable, PropValue::Bool(v), Handle::TabViewItem(tab)) => {
                tab.SetIsClosable(*v)?;
            }
            (Prop::Fill, PropValue::Brush(b), h) => {
                if let Ok(shape) = h.cast_inner::<Xaml::Shape>() {
                    shape.SetFill(&xaml_brush(b)?)?;
                }
            }
            (Prop::Stroke, PropValue::Brush(b), h) => {
                if let Ok(shape) = h.cast_inner::<Xaml::Shape>() {
                    shape.SetStroke(&xaml_brush(b)?)?;
                }
            }
            (Prop::StrokeThickness, PropValue::F64(v), h) => {
                if let Ok(shape) = h.cast_inner::<Xaml::Shape>() {
                    shape.SetStrokeThickness(*v)?;
                }
            }
            (Prop::LineEndpoints, PropValue::LineEndpoints(line), Handle::Line(l)) => {
                l.SetX1(line.x1)?;
                l.SetY1(line.y1)?;
                l.SetX2(line.x2)?;
                l.SetY2(line.y2)?;
            }
            (Prop::CornerRadius, PropValue::F64(v), Handle::Rectangle(r)) => {
                r.SetRadiusX(*v)?;
                r.SetRadiusY(*v)?;
            }
            (Prop::DisplayName, PropValue::Str(s), Handle::PersonPicture(p)) => {
                p.SetDisplayName(&hstring(s))?;
            }
            (Prop::Initials, PropValue::Str(s), Handle::PersonPicture(p)) => {
                p.SetInitials(&hstring(s))?;
            }
            (Prop::IsReadOnly, PropValue::Bool(v), Handle::RichEditBox(reb)) => {
                reb.SetIsReadOnly(*v)?;
            }
            (Prop::AcceptsReturn, PropValue::Bool(v), Handle::TextBox(tb)) => {
                tb.SetAcceptsReturn(*v)?;
            }
            (Prop::PasswordRevealMode, PropValue::I32(v), Handle::PasswordBox(pb)) => {
                pb.SetPasswordRevealMode(Xaml::PasswordRevealMode(*v))?;
            }
            (Prop::IsPasswordRevealButtonEnabled, PropValue::Bool(v), Handle::PasswordBox(pb)) => {
                pb.SetIsPasswordRevealButtonEnabled(*v)?;
            }
            (Prop::DayVisible, PropValue::Bool(v), Handle::DatePicker(dp)) => {
                dp.SetDayVisible(*v)?;
            }
            (Prop::MonthVisible, PropValue::Bool(v), Handle::DatePicker(dp)) => {
                dp.SetMonthVisible(*v)?;
            }
            (Prop::YearVisible, PropValue::Bool(v), Handle::DatePicker(dp)) => {
                dp.SetYearVisible(*v)?;
            }
            (Prop::ClockIdentifier, PropValue::Str(s), Handle::TimePicker(tp)) => {
                tp.SetClockIdentifier(&hstring(s))?;
            }
            (Prop::MinuteIncrement, PropValue::I32(v), Handle::TimePicker(tp)) => {
                tp.SetMinuteIncrement(*v)?;
            }
            (Prop::IsCalendarOpen, PropValue::Bool(v), Handle::CalendarDatePicker(cdp)) => {
                cdp.SetIsCalendarOpen(*v)?;
            }
            (Prop::IsTodayHighlighted, PropValue::Bool(v), h) => {
                if let Ok(cdp) = h.cast_inner::<Xaml::CalendarDatePicker>() {
                    cdp.SetIsTodayHighlighted(*v)?;
                } else if let Ok(cv) = h.cast_inner::<Xaml::CalendarView>() {
                    cv.SetIsTodayHighlighted(*v)?;
                }
            }
            (Prop::IsGroupLabelVisible, PropValue::Bool(v), Handle::CalendarView(cv)) => {
                cv.SetIsGroupLabelVisible(*v)?;
            }
            (Prop::Delay, PropValue::I32(v), Handle::RepeatButton(rb)) => {
                rb.SetDelay(*v)?;
            }
            (Prop::Interval, PropValue::I32(v), Handle::RepeatButton(rb)) => {
                rb.SetInterval(*v)?;
            }
            (Prop::OpenPaneLength, PropValue::F64(v), Handle::SplitView(sv)) => {
                sv.SetOpenPaneLength(*v)?;
            }
            (Prop::CompactPaneLength, PropValue::F64(v), Handle::SplitView(sv)) => {
                sv.SetCompactPaneLength(*v)?;
            }
            (Prop::DisplayMode, PropValue::I32(v), Handle::SplitView(sv)) => {
                sv.SetDisplayMode(Xaml::SplitViewDisplayMode(*v))?;
            }
            (Prop::HorizontalScrollBarVisibility, PropValue::I32(v), h) => {
                if let Ok(sv) = h.cast_inner::<Xaml::ScrollViewer>() {
                    sv.SetHorizontalScrollBarVisibility(Xaml::ScrollBarVisibility(*v))?;
                }
            }
            (Prop::VerticalScrollBarVisibility, PropValue::I32(v), h) => {
                if let Ok(sv) = h.cast_inner::<Xaml::ScrollViewer>() {
                    sv.SetVerticalScrollBarVisibility(Xaml::ScrollBarVisibility(*v))?;
                }
            }
            _ => {}
        }

        Ok(())
    }

    fn visual_insert_at(&self, parent: ControlId, index: usize, child: ControlId) {
        let map = self.controls.borrow();
        let Some(parent_h) = map.get(&parent) else {
            return;
        };
        let Some(child_h) = map.get(&child) else {
            return;
        };
        let Ok(child_ui) = child_h.as_ui_element() else {
            return;
        };

        match parent_h {
            Handle::StackPanel(_)
            | Handle::Grid(_)
            | Handle::Canvas(_)
            | Handle::RelativePanel(_) => {
                if let Some(children) = panel_children_vec(parent_h) {
                    let size = children.Size().unwrap_or(0);
                    if index as u32 >= size {
                        let _ = children.Append(&child_ui);
                    } else {
                        let _ = children.InsertAt(index as u32, &child_ui);
                    }
                }
            }
            Handle::Border(border) => {
                let _ = border.SetChild(&child_ui);
            }
            Handle::Viewbox(v) => {
                let _ = v.SetChild(&child_ui);
            }
            Handle::ScrollViewer(_)
            | Handle::NavigationView(_)
            | Handle::PivotItem(_)
            | Handle::Expander(_)
            | Handle::TabViewItem(_)
            | Handle::TeachingTip(_) => {
                if let Some(cc) = content_control_for(parent_h) {
                    let _ = cc.SetContent(&child_ui);
                }
            }
            Handle::SplitView(sv) => {
                let _ = sv.SetContent(&child_ui);
            }
            Handle::Pivot(pivot) => {
                if let Ok(items) = pivot
                    .Items()
                    .and_then(|i| i.cast::<windows_collections::IVector<IInspectable>>())
                {
                    let Ok(insp) = child_ui.cast::<IInspectable>() else {
                        return;
                    };
                    let size = items.Size().unwrap_or(0);
                    if index as u32 >= size {
                        let _ = items.Append(&insp);
                    } else {
                        let _ = items.InsertAt(index as u32, &insp);
                    }
                }
            }
            Handle::TabView(tab) => {
                if let Ok(items) = tab.TabItems() {
                    let Ok(insp) = child_ui.cast::<IInspectable>() else {
                        return;
                    };
                    let size = items.Size().unwrap_or(0);
                    if index as u32 >= size {
                        let _ = items.Append(&insp);
                    } else {
                        let _ = items.InsertAt(index as u32, &insp);
                    }
                }
            }
            Handle::ListView(_) | Handle::GridView(_) | Handle::FlipView(_) => {
                if let Some(items) = items_vector(parent_h) {
                    let Ok(insp) = child_ui.cast::<IInspectable>() else {
                        return;
                    };
                    let size = items.Size().unwrap_or(0);
                    if index as u32 >= size {
                        let _ = items.Append(&insp);
                    } else {
                        let _ = items.InsertAt(index as u32, &insp);
                    }
                }
            }
            _ => {}
        }
    }

    fn visual_remove_at(&self, parent: ControlId, index: usize) {
        let map = self.controls.borrow();
        let Some(parent_h) = map.get(&parent) else {
            return;
        };
        match parent_h {
            Handle::StackPanel(_)
            | Handle::Grid(_)
            | Handle::Canvas(_)
            | Handle::RelativePanel(_) => {
                if let Some(children) = panel_children_vec(parent_h)
                    && (index as u32) < children.Size().unwrap_or(0)
                {
                    let _ = children.RemoveAt(index as u32);
                }
            }
            Handle::Border(border) => {
                let _ = border.SetChild(None);
            }
            Handle::Viewbox(v) => {
                let _ = v.SetChild(None);
            }
            Handle::ScrollViewer(_)
            | Handle::NavigationView(_)
            | Handle::PivotItem(_)
            | Handle::Expander(_)
            | Handle::TabViewItem(_)
            | Handle::TeachingTip(_) => {
                if let Some(cc) = content_control_for(parent_h) {
                    let _ = cc.SetContent(None);
                }
            }
            Handle::SplitView(sv) => {
                let _ = sv.SetContent(None);
            }
            Handle::Pivot(_) | Handle::ListView(_) | Handle::GridView(_) | Handle::FlipView(_) => {
                if let Some(items) = items_vector(parent_h)
                    && (index as u32) < items.Size().unwrap_or(0)
                {
                    let _ = items.RemoveAt(index as u32);
                }
            }
            Handle::TabView(tab) => {
                if let Ok(items) = tab.TabItems()
                    && (index as u32) < items.Size().unwrap_or(0)
                {
                    let _ = items.RemoveAt(index as u32);
                }
            }
            _ => {}
        }
    }

    fn visual_set_at(&self, parent: ControlId, index: usize, child: ControlId) {
        self.visual_remove_at(parent, index);
        self.visual_insert_at(parent, index, child);
    }
}

impl Backend for WinUIBackend {
    fn create(&mut self, kind: ControlKind) -> ControlId {
        let id = self.alloc_id();
        let handle = Handle::new(kind).unwrap_or_else(|err| {
            panic!("island-reactor: unsupported or failed native control {kind:?}: {err:?}")
        });
        self.controls.borrow_mut().insert(id, handle);
        id
    }

    fn set_prop(&mut self, id: ControlId, prop: Prop, value: &PropValue) {
        let _ = self.set_prop_inner(id, prop, value);
    }

    fn append_child(&mut self, parent: ControlId, child: ControlId) {
        let index = self
            .parent_children
            .borrow()
            .get(&parent)
            .map_or(0, Vec::len);
        self.insert_child(parent, index, child);
    }

    fn remove_child(&mut self, parent: ControlId, index: usize) {
        let visual = self.visual_index(parent, index);
        if let Some(list) = self.parent_children.borrow_mut().get_mut(&parent)
            && index < list.len()
        {
            list.remove(index);
        }
        self.visual_remove_at(parent, visual);
    }

    fn replace_child(&mut self, parent: ControlId, index: usize, new: ControlId) {
        let visual = self.visual_index(parent, index);
        if let Some(list) = self.parent_children.borrow_mut().get_mut(&parent)
            && index < list.len()
        {
            list[index] = new;
        }
        self.visual_set_at(parent, visual, new);
    }

    fn move_child(&mut self, parent: ControlId, from: usize, to: usize) {
        let child = {
            let mut parents = self.parent_children.borrow_mut();
            let Some(list) = parents.get_mut(&parent) else {
                return;
            };
            if from >= list.len() || to >= list.len() {
                return;
            }
            let child = list.remove(from);
            list.insert(to, child);
            child
        };
        self.visual_remove_at(parent, from);
        self.visual_insert_at(parent, to, child);
    }

    fn insert_child(&mut self, parent: ControlId, index: usize, child: ControlId) {
        let visual = self.visual_index(parent, index);
        self.parent_children
            .borrow_mut()
            .entry(parent)
            .or_default()
            .insert(index, child);
        self.visual_insert_at(parent, visual, child);
    }

    fn destroy(&mut self, id: ControlId) {
        self.controls.borrow_mut().remove(&id);
        self.event_revokers
            .borrow_mut()
            .retain(|(cid, _), _| *cid != id);
        self.templated_selection_revokers.borrow_mut().remove(&id);
        self.templated_realizers.borrow_mut().remove(&id);
        self.parent_children.borrow_mut().remove(&id);
        self.templated_rows.borrow_mut().remove(&id);
    }

    fn attach_event(&mut self, id: ControlId, event: Event, handler: EventHandler) {
        self.detach_event(id, event);
        let mut revokers = Vec::new();
        let map = self.controls.borrow();
        let Some(handle) = map.get(&id) else {
            return;
        };

        match (event, handle) {
            (Event::ItemClicked, Handle::BreadcrumbBar(breadcrumb)) => {
                let event_handler = windows::Foundation::TypedEventHandler::<
                    Muxc::BreadcrumbBar,
                    Muxc::BreadcrumbBarItemClickedEventArgs,
                >::new(move |_, args| {
                    let index = args.as_ref().and_then(|a| a.Index().ok()).unwrap_or(-1);
                    handler.invoke_i32(index);
                    Ok(())
                });
                let token = breadcrumb.ItemClicked(&event_handler).unwrap();
                revokers.push(EventSubscription::BreadcrumbBarItemClicked(
                    breadcrumb.clone(),
                    token,
                ));
            }
            (Event::Click, h) => {
                if let Ok(button) = h.cast_inner::<Xaml::ButtonBase>() {
                    let event_handler = Xaml::RoutedEventHandler::new(move |_, _| {
                        handler.invoke();
                        Ok(())
                    });
                    let token = button.Click(&event_handler).unwrap();
                    revokers.push(EventSubscription::ButtonClick(button, token));
                }
            }
            (Event::Checked, h) => {
                if let Ok(toggle) = h.cast_inner::<Xaml::ToggleButton>() {
                    let h1 = handler.clone();
                    let checked_handler = Xaml::RoutedEventHandler::new(move |_, _| {
                        h1.invoke_bool(true);
                        Ok(())
                    });
                    let unchecked_handler = Xaml::RoutedEventHandler::new(move |_, _| {
                        handler.invoke_bool(false);
                        Ok(())
                    });
                    let checked = toggle.Checked(&checked_handler).unwrap();
                    let unchecked = toggle.Unchecked(&unchecked_handler).unwrap();
                    revokers.push(EventSubscription::ToggleButtonChecked(
                        toggle.clone(),
                        checked,
                    ));
                    revokers.push(EventSubscription::ToggleButtonUnchecked(toggle, unchecked));
                }
            }
            (Event::Toggled, Handle::ToggleSwitch(ts)) => {
                let ts = ts.clone();
                let ts_for_cb = ts.clone();
                let handler = Xaml::RoutedEventHandler::new(move |_, _| {
                    handler.invoke_bool(ts_for_cb.IsOn().unwrap_or(false));
                    Ok(())
                });
                let token = ts.Toggled(&handler).unwrap();
                revokers.push(EventSubscription::ToggleSwitchToggled(ts, token));
            }
            (Event::Expanding, Handle::Expander(expander)) => {
                let h1 = handler.clone();
                let expanding_handler = windows::Foundation::TypedEventHandler::<
                    Muxc::Expander,
                    Muxc::ExpanderExpandingEventArgs,
                >::new(move |_, _| {
                    h1.invoke_bool(true);
                    Ok(())
                });
                let collapsed_handler = windows::Foundation::TypedEventHandler::<
                    Muxc::Expander,
                    Muxc::ExpanderCollapsedEventArgs,
                >::new(move |_, _| {
                    handler.invoke_bool(false);
                    Ok(())
                });
                let expanding = expander.Expanding(&expanding_handler).unwrap();
                let collapsed = expander.Collapsed(&collapsed_handler).unwrap();
                revokers.push(EventSubscription::ExpanderExpanding(
                    expander.clone(),
                    expanding,
                ));
                revokers.push(EventSubscription::ExpanderCollapsed(
                    expander.clone(),
                    collapsed,
                ));
            }
            (Event::TextChanged, Handle::TextBox(tb)) => {
                let handler = Xaml::TextChangedEventHandler::new(move |sender, _| {
                    let text = sender
                        .as_ref()
                        .and_then(|s| s.cast::<Xaml::TextBox>().ok())
                        .and_then(|t| t.Text().ok())
                        .map(|s| s.to_string_lossy())
                        .unwrap_or_default();
                    handler.invoke_string(text);
                    Ok(())
                });
                let token = tb.TextChanged(&handler).unwrap();
                revokers.push(EventSubscription::TextBoxTextChanged(tb.clone(), token));
            }
            (Event::TextChanged, Handle::AutoSuggestBox(asb)) => {
                let handler = windows::Foundation::TypedEventHandler::<
                    Xaml::AutoSuggestBox,
                    Xaml::AutoSuggestBoxTextChangedEventArgs,
                >::new(move |sender, _| {
                    let text = sender
                        .as_ref()
                        .and_then(|t| t.Text().ok())
                        .map(|s| s.to_string_lossy())
                        .unwrap_or_default();
                    handler.invoke_string(text);
                    Ok(())
                });
                let token = asb.TextChanged(&handler).unwrap();
                revokers.push(EventSubscription::AutoSuggestBoxTextChanged(
                    asb.clone(),
                    token,
                ));
            }
            (Event::QuerySubmitted, Handle::AutoSuggestBox(asb)) => {
                let handler = windows::Foundation::TypedEventHandler::<
                    Xaml::AutoSuggestBox,
                    Xaml::AutoSuggestBoxQuerySubmittedEventArgs,
                >::new(move |_, args| {
                    let text = args
                        .as_ref()
                        .and_then(|a| a.QueryText().ok())
                        .map(|s| s.to_string_lossy())
                        .unwrap_or_default();
                    handler.invoke_string(text);
                    Ok(())
                });
                let token = asb.QuerySubmitted(&handler).unwrap();
                revokers.push(EventSubscription::AutoSuggestBoxQuerySubmitted(
                    asb.clone(),
                    token,
                ));
            }
            (Event::SuggestionChosen, Handle::AutoSuggestBox(asb)) => {
                let handler = windows::Foundation::TypedEventHandler::<
                    Xaml::AutoSuggestBox,
                    Xaml::AutoSuggestBoxSuggestionChosenEventArgs,
                >::new(move |_, args| {
                    let text = args
                        .as_ref()
                        .and_then(|a| a.SelectedItem().ok())
                        .and_then(inspectable_to_string)
                        .unwrap_or_default();
                    handler.invoke_string(text);
                    Ok(())
                });
                let token = asb.SuggestionChosen(&handler).unwrap();
                revokers.push(EventSubscription::AutoSuggestBoxSuggestionChosen(
                    asb.clone(),
                    token,
                ));
            }
            (Event::SelectionChanged, h) => {
                if let Ok(selector) = h.cast_inner::<Xaml::Selector>() {
                    let selector_for_cb = selector.clone();
                    let handler = Xaml::SelectionChangedEventHandler::new(move |_, _| {
                        handler.invoke_i32(selector_for_cb.SelectedIndex().unwrap_or(-1));
                        Ok(())
                    });
                    let token = selector.SelectionChanged(&handler).unwrap();
                    revokers.push(EventSubscription::SelectorSelectionChanged(selector, token));
                } else if let Ok(radio) = h.cast_inner::<Muxc::RadioButtons>() {
                    let radio_for_cb = radio.clone();
                    let handler = Xaml::SelectionChangedEventHandler::new(move |_, _| {
                        handler.invoke_i32(radio_for_cb.SelectedIndex().unwrap_or(-1));
                        Ok(())
                    });
                    let token = radio.SelectionChanged(&handler).unwrap();
                    revokers.push(EventSubscription::RadioButtonsSelectionChanged(
                        radio, token,
                    ));
                } else if let Ok(tab) = h.cast_inner::<Muxc::TabView>() {
                    let tab_for_cb = tab.clone();
                    let handler = Xaml::SelectionChangedEventHandler::new(move |_, _| {
                        handler.invoke_i32(tab_for_cb.SelectedIndex().unwrap_or(-1));
                        Ok(())
                    });
                    let token = tab.SelectionChanged(&handler).unwrap();
                    revokers.push(EventSubscription::TabViewSelectionChanged(tab, token));
                } else if let Ok(nav) = h.cast_inner::<Muxc::NavigationView>() {
                    let handler = windows::Foundation::TypedEventHandler::<
                        Muxc::NavigationView,
                        Muxc::NavigationViewSelectionChangedEventArgs,
                    >::new(move |_, args| {
                        let tag = args
                            .as_ref()
                            .and_then(|a| a.SelectedItem().ok())
                            .and_then(|i| i.cast::<Xaml::FrameworkElement>().ok())
                            .and_then(|fe| fe.Tag().ok())
                            .and_then(inspectable_to_string)
                            .unwrap_or_default();
                        handler.invoke_string(tag);
                        Ok(())
                    });
                    let token = nav.SelectionChanged(&handler).unwrap();
                    revokers.push(EventSubscription::NavigationViewSelectionChanged(
                        nav, token,
                    ));
                }
            }
            (Event::ValueChanged, h) => {
                if let Ok(range) = h.cast_inner::<Xaml::RangeBase>() {
                    let handler = Xaml::RangeBaseValueChangedEventHandler::new(move |_, args| {
                        let value = args
                            .as_ref()
                            .and_then(|a| a.NewValue().ok())
                            .unwrap_or_default();
                        handler.invoke_f64(value);
                        Ok(())
                    });
                    let token = range.ValueChanged(&handler).unwrap();
                    revokers.push(EventSubscription::RangeBaseValueChanged(range, token));
                } else if let Ok(rating) = h.cast_inner::<Xaml::RatingControl>() {
                    let handler = windows::Foundation::TypedEventHandler::<
                        Xaml::RatingControl,
                        IInspectable,
                    >::new(move |sender, _| {
                        let value = sender
                            .as_ref()
                            .and_then(|r| r.Value().ok())
                            .unwrap_or_default();
                        handler.invoke_f64(value);
                        Ok(())
                    });
                    let token = rating.ValueChanged(&handler).unwrap();
                    revokers.push(EventSubscription::RatingControlValueChanged(rating, token));
                } else if let Ok(number) = h.cast_inner::<Muxc::NumberBox>() {
                    let handler = windows::Foundation::TypedEventHandler::<
                        Muxc::NumberBox,
                        Muxc::NumberBoxValueChangedEventArgs,
                    >::new(move |_, args| {
                        let value = args
                            .as_ref()
                            .and_then(|a| a.NewValue().ok())
                            .unwrap_or_default();
                        handler.invoke_f64(value);
                        Ok(())
                    });
                    let token = number.ValueChanged(&handler).unwrap();
                    revokers.push(EventSubscription::NumberBoxValueChanged(number, token));
                }
            }
            (Event::CloseRequested, Handle::TabView(tab)) => {
                let event_handler = windows::Foundation::TypedEventHandler::<
                    Muxc::TabView,
                    Muxc::TabViewTabCloseRequestedEventArgs,
                >::new(move |_, args| {
                    let key = args
                        .as_ref()
                        .and_then(|a| a.Tab().ok())
                        .and_then(|tab| tab.cast::<Xaml::FrameworkElement>().ok())
                        .and_then(|fe| fe.Tag().ok())
                        .and_then(inspectable_to_string)
                        .unwrap_or_default();
                    handler.invoke_string(key);
                    Ok(())
                });
                let token = tab.TabCloseRequested(&event_handler).unwrap();
                revokers.push(EventSubscription::TabViewCloseRequested(tab.clone(), token));
            }
            (Event::AddTabButtonClick, Handle::TabView(tab)) => {
                let event_handler = windows::Foundation::TypedEventHandler::<
                    Muxc::TabView,
                    IInspectable,
                >::new(move |_, _| {
                    handler.invoke();
                    Ok(())
                });
                let token = tab.AddTabButtonClick(&event_handler).unwrap();
                revokers.push(EventSubscription::TabViewAddTabButtonClick(
                    tab.clone(),
                    token,
                ));
            }
            (Event::Closed, Handle::InfoBar(info)) => {
                let event_handler = windows::Foundation::TypedEventHandler::<
                    Muxc::InfoBar,
                    Muxc::InfoBarClosedEventArgs,
                >::new(move |_, _| {
                    handler.invoke();
                    Ok(())
                });
                let token = info.Closed(&event_handler).unwrap();
                revokers.push(EventSubscription::InfoBarClosed(info.clone(), token));
            }
            (Event::Closed, Handle::TeachingTip(tip)) => {
                let event_handler = windows::Foundation::TypedEventHandler::<
                    Muxc::TeachingTip,
                    Muxc::TeachingTipClosedEventArgs,
                >::new(move |_, _| {
                    handler.invoke();
                    Ok(())
                });
                let token = tip.Closed(&event_handler).unwrap();
                revokers.push(EventSubscription::TeachingTipClosed(tip.clone(), token));
            }
            (Event::ActionButtonClick, Handle::TeachingTip(tip)) => {
                let event_handler = windows::Foundation::TypedEventHandler::<
                    Muxc::TeachingTip,
                    IInspectable,
                >::new(move |_, _| {
                    handler.invoke();
                    Ok(())
                });
                let token = tip.ActionButtonClick(&event_handler).unwrap();
                revokers.push(EventSubscription::TeachingTipActionButtonClick(
                    tip.clone(),
                    token,
                ));
            }
            (Event::BackRequested, Handle::NavigationView(nav)) => {
                let event_handler = windows::Foundation::TypedEventHandler::<
                    Muxc::NavigationView,
                    Muxc::NavigationViewBackRequestedEventArgs,
                >::new(move |_, _| {
                    handler.invoke();
                    Ok(())
                });
                let token = nav.BackRequested(&event_handler).unwrap();
                revokers.push(EventSubscription::NavigationViewBackRequested(
                    nav.clone(),
                    token,
                ));
            }
            (Event::PasswordChanged, Handle::PasswordBox(pb)) => {
                let event_handler = Xaml::RoutedEventHandler::new(move |sender, _| {
                    let text = sender
                        .as_ref()
                        .and_then(|s| s.cast::<Xaml::PasswordBox>().ok())
                        .and_then(|p| p.Password().ok())
                        .map(|s| s.to_string_lossy())
                        .unwrap_or_default();
                    handler.invoke_string(text);
                    Ok(())
                });
                let token = pb.PasswordChanged(&event_handler).unwrap();
                revokers.push(EventSubscription::PasswordBoxPasswordChanged(
                    pb.clone(),
                    token,
                ));
            }
            _ => {}
        }

        if !revokers.is_empty() {
            self.event_revokers
                .borrow_mut()
                .insert((id, event), revokers);
        }
    }

    fn detach_event(&mut self, id: ControlId, event: Event) {
        self.event_revokers.borrow_mut().remove(&(id, event));
    }

    fn set_templated_item_count(&mut self, id: ControlId, count: usize) {
        self.templated_rows
            .borrow_mut()
            .entry(id)
            .or_default()
            .resize(count, None);
        if let Some(realize) = self.templated_realizers.borrow().get(&id).cloned() {
            for row in 0..count {
                realize(row);
            }
        }
    }

    fn set_templated_row_content(
        &mut self,
        list_id: ControlId,
        row_idx: usize,
        content: Option<ControlId>,
    ) {
        let mut rows = self.templated_rows.borrow_mut();
        let list = rows.entry(list_id).or_default();
        if row_idx >= list.len() {
            list.resize(row_idx + 1, None);
        }
        list[row_idx] = content;
        drop(rows);

        let Some(content_id) = content else {
            return;
        };
        self.visual_insert_at(list_id, row_idx, content_id);
    }

    fn set_templated_selected_index(&mut self, id: ControlId, index: i32) {
        let _ = self.set_prop_inner(id, Prop::SelectedIndex, &PropValue::I32(index));
    }

    fn set_templated_selection_mode(
        &mut self,
        id: ControlId,
        mode: crate::core::templated_list::SelectionMode,
    ) {
        let native = match mode {
            crate::core::templated_list::SelectionMode::None => 0,
            crate::core::templated_list::SelectionMode::Single => 1,
            crate::core::templated_list::SelectionMode::Multiple => 2,
            crate::core::templated_list::SelectionMode::Extended => 3,
        };
        let _ = self.set_prop_inner(id, Prop::SelectionMode, &PropValue::I32(native));
    }

    fn attach_templated_selection_changed(&mut self, id: ControlId, handler: Callback<i32>) {
        let map = self.controls.borrow();
        let Some(handle) = map.get(&id) else {
            return;
        };
        if let Ok(selector) = handle.cast_inner::<Xaml::Selector>() {
            let selector_for_cb = selector.clone();
            let event_handler = Xaml::SelectionChangedEventHandler::new(move |_, _| {
                handler.invoke(selector_for_cb.SelectedIndex().unwrap_or(-1));
                Ok(())
            });
            if let Ok(revoker) = selector.SelectionChanged(&event_handler) {
                let subscription = EventSubscription::SelectorSelectionChanged(selector, revoker);
                self.templated_selection_revokers
                    .borrow_mut()
                    .insert(id, subscription);
            }
        }
    }

    fn attach_templated_realization(
        &mut self,
        id: ControlId,
        realize: Rc<dyn Fn(usize)>,
        _recycle: Rc<dyn Fn(usize)>,
    ) {
        self.templated_realizers.borrow_mut().insert(id, realize);
    }

    fn set_header_element(&mut self, id: ControlId, header_id: Option<ControlId>) {
        let map = self.controls.borrow();
        let Some(Handle::Expander(expander)) = map.get(&id) else {
            return;
        };
        match header_id
            .and_then(|hid| map.get(&hid))
            .and_then(|h| h.as_ui_element().ok())
            .and_then(|ui| ui.cast::<IInspectable>().ok())
        {
            Some(header) => {
                let _ = expander.SetHeader(&header);
            }
            None => {
                let _ = expander.SetHeader(None);
            }
        }
    }

    fn set_pane_element(&mut self, id: ControlId, pane_id: Option<ControlId>) {
        let map = self.controls.borrow();
        let Some(Handle::SplitView(split)) = map.get(&id) else {
            return;
        };
        match pane_id
            .and_then(|pid| map.get(&pid))
            .and_then(|h| h.as_ui_element().ok())
        {
            Some(ui) => {
                let _ = split.SetPane(&ui);
            }
            None => {
                let _ = split.SetPane(None);
            }
        }
    }

    fn set_theme_bindings(
        &mut self,
        _id: ControlId,
        _kind: ControlKind,
        _bindings: &[(Prop, crate::core::theme::ThemeRef)],
    ) {
    }

    fn set_implicit_transitions(
        &mut self,
        _id: ControlId,
        _transitions: Option<crate::core::animation::ImplicitTransitions>,
    ) {
    }

    fn run_property_animation(
        &mut self,
        _id: ControlId,
        _config: Option<crate::core::animation::AnimationConfig>,
    ) {
    }

    fn set_rich_text_paragraphs(
        &mut self,
        id: ControlId,
        paragraphs: &[crate::core::rich_text::RichTextParagraph],
    ) {
        let map = self.controls.borrow();
        let Some(Handle::RichTextBlock(rtb)) = map.get(&id) else {
            return;
        };
        let Ok(blocks) = rtb.Blocks() else {
            return;
        };
        let _ = blocks.Clear();
        for para_def in paragraphs {
            let Ok(para) = Xaml::Paragraph::new() else {
                continue;
            };
            let Ok(inlines) = para.Inlines() else {
                continue;
            };
            for inline in &para_def.inlines {
                let text = match inline {
                    crate::core::rich_text::RichTextInline::Run(run) => run.text.as_str(),
                    crate::core::rich_text::RichTextInline::LineBreak => "\n",
                    crate::core::rich_text::RichTextInline::Hyperlink(link) => link.text.as_str(),
                };
                if let Ok(run) = Xaml::Run::new() {
                    let _ = run.SetText(&hstring(text));
                    let _ = run.cast::<Xaml::Inline>().and_then(|r| inlines.Append(&r));
                }
            }
            let _ = para.cast::<Xaml::Block>().and_then(|p| blocks.Append(&p));
        }
    }

    fn set_tooltip(&mut self, id: ControlId, tooltip: Option<&crate::core::tooltip::Tooltip>) {
        let map = self.controls.borrow();
        let Some(handle) = map.get(&id) else {
            return;
        };
        let Ok(dep) = handle
            .as_framework_element()
            .and_then(|fe| fe.cast::<Xaml::DependencyObject>())
        else {
            return;
        };
        let inspectable = tooltip.and_then(|t| match &t.content {
            crate::core::tooltip::TooltipContent::Text(s) => Some(string_reference(s)),
            crate::core::tooltip::TooltipContent::Rich(_) => None,
        });
        let _ = Xaml::ToolTipService::SetToolTip(&dep, inspectable.as_ref());
    }

    fn get_native_element(&self, id: ControlId) -> Option<IInspectable> {
        self.get_ui_element(id)
    }
}

enum Handle {
    AutoSuggestBox(Xaml::AutoSuggestBox),
    Border(Xaml::Border),
    BreadcrumbBar(Muxc::BreadcrumbBar),
    Button(Xaml::Button),
    CalendarDatePicker(Xaml::CalendarDatePicker),
    CalendarView(Xaml::CalendarView),
    Canvas(Xaml::Canvas),
    CheckBox(Xaml::CheckBox),
    ColorPicker(Xaml::ColorPicker),
    ComboBox(Xaml::ComboBox),
    CommandBar(Xaml::CommandBar),
    ContentDialog(Xaml::ContentDialog),
    DatePicker(Xaml::DatePicker),
    DropDownButton(Xaml::DropDownButton),
    Expander(Muxc::Expander),
    Grid(Xaml::Grid),
    HyperlinkButton(Xaml::HyperlinkButton),
    Image(Xaml::Image),
    InfoBadge(Muxc::InfoBadge),
    InfoBar(Muxc::InfoBar),
    ListBox(Xaml::ListBox),
    MenuBar(Xaml::MenuBar),
    NavigationView(Muxc::NavigationView),
    NumberBox(Muxc::NumberBox),
    PasswordBox(Xaml::PasswordBox),
    PersonPicture(Xaml::PersonPicture),
    Pivot(Xaml::Pivot),
    PivotItem(Xaml::PivotItem),
    ProgressBar(Xaml::ProgressBar),
    ProgressRing(Xaml::ProgressRing),
    RadioButton(Xaml::RadioButton),
    RadioButtons(Muxc::RadioButtons),
    RatingControl(Xaml::RatingControl),
    Rectangle(Xaml::Rectangle),
    RelativePanel(Xaml::RelativePanel),
    RepeatButton(Xaml::RepeatButton),
    RichEditBox(Xaml::RichEditBox),
    RichTextBlock(Xaml::RichTextBlock),
    ScrollViewer(Xaml::ScrollViewer),
    Slider(Xaml::Slider),
    SplitButton(Xaml::SplitButton),
    SplitView(Xaml::SplitView),
    StackPanel(Xaml::StackPanel),
    TabView(Muxc::TabView),
    TabViewItem(Muxc::TabViewItem),
    TeachingTip(Muxc::TeachingTip),
    TextBlock(Xaml::TextBlock),
    TextBox(Xaml::TextBox),
    TimePicker(Xaml::TimePicker),
    ToggleButton(Xaml::ToggleButton),
    ToggleSwitch(Xaml::ToggleSwitch),
    TreeView(Xaml::TreeView),
    Viewbox(Xaml::Viewbox),
    Ellipse(Xaml::Ellipse),
    Line(Xaml::Line),
    ListView(Xaml::ListView),
    GridView(Xaml::GridView),
    FlipView(Xaml::FlipView),
}

impl Handle {
    fn new(kind: ControlKind) -> Result<Self> {
        Ok(match kind {
            ControlKind::AutoSuggestBox => Self::AutoSuggestBox(Xaml::AutoSuggestBox::new()?),
            ControlKind::Border => Self::Border(Xaml::Border::new()?),
            ControlKind::BreadcrumbBar => Self::BreadcrumbBar(Muxc::BreadcrumbBar::new()?),
            ControlKind::Button => Self::Button(Xaml::Button::new()?),
            ControlKind::CalendarDatePicker => {
                Self::CalendarDatePicker(Xaml::CalendarDatePicker::new()?)
            }
            ControlKind::CalendarView => Self::CalendarView(Xaml::CalendarView::new()?),
            ControlKind::Canvas => Self::Canvas(Xaml::Canvas::new()?),
            ControlKind::CheckBox => Self::CheckBox(Xaml::CheckBox::new()?),
            ControlKind::ColorPicker => Self::ColorPicker(Xaml::ColorPicker::new()?),
            ControlKind::ComboBox => Self::ComboBox(Xaml::ComboBox::new()?),
            ControlKind::CommandBar => Self::CommandBar(Xaml::CommandBar::new()?),
            ControlKind::ContentDialog => Self::ContentDialog(Xaml::ContentDialog::new()?),
            ControlKind::DatePicker => Self::DatePicker(Xaml::DatePicker::new()?),
            ControlKind::DropDownButton => Self::DropDownButton(Xaml::DropDownButton::new()?),
            ControlKind::Expander => Self::Expander(Muxc::Expander::new()?),
            ControlKind::Grid => Self::Grid(Xaml::Grid::new()?),
            ControlKind::HyperlinkButton => Self::HyperlinkButton(Xaml::HyperlinkButton::new()?),
            ControlKind::Image => Self::Image(Xaml::Image::new()?),
            ControlKind::InfoBadge => Self::InfoBadge(Muxc::InfoBadge::new()?),
            ControlKind::InfoBar => Self::InfoBar(Muxc::InfoBar::new()?),
            ControlKind::ListBox => Self::ListBox(Xaml::ListBox::new()?),
            ControlKind::MenuBar => Self::MenuBar(Xaml::MenuBar::new()?),
            ControlKind::NavigationView => Self::NavigationView(Muxc::NavigationView::new()?),
            ControlKind::NumberBox => Self::NumberBox(Muxc::NumberBox::new()?),
            ControlKind::PasswordBox => Self::PasswordBox(Xaml::PasswordBox::new()?),
            ControlKind::PersonPicture => Self::PersonPicture(Xaml::PersonPicture::new()?),
            ControlKind::Pivot => Self::Pivot(Xaml::Pivot::new()?),
            ControlKind::PivotItem => Self::PivotItem(Xaml::PivotItem::new()?),
            ControlKind::ProgressBar => Self::ProgressBar(Xaml::ProgressBar::new()?),
            ControlKind::ProgressRing => Self::ProgressRing(Xaml::ProgressRing::new()?),
            ControlKind::RadioButton => Self::RadioButton(Xaml::RadioButton::new()?),
            ControlKind::RadioButtons => Self::RadioButtons(Muxc::RadioButtons::new()?),
            ControlKind::RatingControl => Self::RatingControl(Xaml::RatingControl::new()?),
            ControlKind::Rectangle => Self::Rectangle(Xaml::Rectangle::new()?),
            ControlKind::RelativePanel => Self::RelativePanel(Xaml::RelativePanel::new()?),
            ControlKind::RepeatButton => Self::RepeatButton(Xaml::RepeatButton::new()?),
            ControlKind::RichEditBox => Self::RichEditBox(Xaml::RichEditBox::new()?),
            ControlKind::RichTextBlock => Self::RichTextBlock(Xaml::RichTextBlock::new()?),
            ControlKind::ScrollViewer => Self::ScrollViewer(Xaml::ScrollViewer::new()?),
            ControlKind::Slider => Self::Slider(Xaml::Slider::new()?),
            ControlKind::SplitButton => Self::SplitButton(Xaml::SplitButton::new()?),
            ControlKind::SplitView => Self::SplitView(Xaml::SplitView::new()?),
            ControlKind::StackPanel => Self::StackPanel(Xaml::StackPanel::new()?),
            ControlKind::TabView => Self::TabView(Muxc::TabView::new()?),
            ControlKind::TabViewItem => Self::TabViewItem(Muxc::TabViewItem::new()?),
            ControlKind::TeachingTip => Self::TeachingTip(Muxc::TeachingTip::new()?),
            ControlKind::TextBlock => Self::TextBlock(Xaml::TextBlock::new()?),
            ControlKind::TextBox => Self::TextBox(Xaml::TextBox::new()?),
            ControlKind::TimePicker => Self::TimePicker(Xaml::TimePicker::new()?),
            ControlKind::ToggleButton => Self::ToggleButton(Xaml::ToggleButton::new()?),
            ControlKind::ToggleSwitch => Self::ToggleSwitch(Xaml::ToggleSwitch::new()?),
            ControlKind::TreeView => Self::TreeView(Xaml::TreeView::new()?),
            ControlKind::Viewbox => Self::Viewbox(Xaml::Viewbox::new()?),
            ControlKind::Ellipse => Self::Ellipse(Xaml::Ellipse::new()?),
            ControlKind::Line => Self::Line(Xaml::Line::new()?),
            ControlKind::ListView => Self::ListView(Xaml::ListView::new()?),
            ControlKind::GridView => Self::GridView(Xaml::GridView::new()?),
            ControlKind::FlipView => Self::FlipView(Xaml::FlipView::new()?),
        })
    }

    fn cast_inner<T: Interface>(&self) -> Result<T> {
        match self {
            Self::AutoSuggestBox(v) => v.cast(),
            Self::Border(v) => v.cast(),
            Self::BreadcrumbBar(v) => v.cast(),
            Self::Button(v) => v.cast(),
            Self::CalendarDatePicker(v) => v.cast(),
            Self::CalendarView(v) => v.cast(),
            Self::Canvas(v) => v.cast(),
            Self::CheckBox(v) => v.cast(),
            Self::ColorPicker(v) => v.cast(),
            Self::ComboBox(v) => v.cast(),
            Self::CommandBar(v) => v.cast(),
            Self::ContentDialog(v) => v.cast(),
            Self::DatePicker(v) => v.cast(),
            Self::DropDownButton(v) => v.cast(),
            Self::Expander(v) => v.cast(),
            Self::Grid(v) => v.cast(),
            Self::HyperlinkButton(v) => v.cast(),
            Self::Image(v) => v.cast(),
            Self::InfoBadge(v) => v.cast(),
            Self::InfoBar(v) => v.cast(),
            Self::ListBox(v) => v.cast(),
            Self::MenuBar(v) => v.cast(),
            Self::NavigationView(v) => v.cast(),
            Self::NumberBox(v) => v.cast(),
            Self::PasswordBox(v) => v.cast(),
            Self::PersonPicture(v) => v.cast(),
            Self::Pivot(v) => v.cast(),
            Self::PivotItem(v) => v.cast(),
            Self::ProgressBar(v) => v.cast(),
            Self::ProgressRing(v) => v.cast(),
            Self::RadioButton(v) => v.cast(),
            Self::RadioButtons(v) => v.cast(),
            Self::RatingControl(v) => v.cast(),
            Self::Rectangle(v) => v.cast(),
            Self::RelativePanel(v) => v.cast(),
            Self::RepeatButton(v) => v.cast(),
            Self::RichEditBox(v) => v.cast(),
            Self::RichTextBlock(v) => v.cast(),
            Self::ScrollViewer(v) => v.cast(),
            Self::Slider(v) => v.cast(),
            Self::SplitButton(v) => v.cast(),
            Self::SplitView(v) => v.cast(),
            Self::StackPanel(v) => v.cast(),
            Self::TabView(v) => v.cast(),
            Self::TabViewItem(v) => v.cast(),
            Self::TeachingTip(v) => v.cast(),
            Self::TextBlock(v) => v.cast(),
            Self::TextBox(v) => v.cast(),
            Self::TimePicker(v) => v.cast(),
            Self::ToggleButton(v) => v.cast(),
            Self::ToggleSwitch(v) => v.cast(),
            Self::TreeView(v) => v.cast(),
            Self::Viewbox(v) => v.cast(),
            Self::Ellipse(v) => v.cast(),
            Self::Line(v) => v.cast(),
            Self::ListView(v) => v.cast(),
            Self::GridView(v) => v.cast(),
            Self::FlipView(v) => v.cast(),
        }
    }

    fn as_ui_element(&self) -> Result<Xaml::UIElement> {
        self.cast_inner()
    }

    fn as_framework_element(&self) -> Result<Xaml::FrameworkElement> {
        self.cast_inner()
    }
}

enum EventSubscription {
    BreadcrumbBarItemClicked(Muxc::BreadcrumbBar, i64),
    ButtonClick(Xaml::ButtonBase, i64),
    ExpanderCollapsed(Muxc::Expander, i64),
    ExpanderExpanding(Muxc::Expander, i64),
    InfoBarClosed(Muxc::InfoBar, i64),
    ToggleButtonChecked(Xaml::ToggleButton, i64),
    ToggleButtonUnchecked(Xaml::ToggleButton, i64),
    ToggleSwitchToggled(Xaml::ToggleSwitch, i64),
    NumberBoxValueChanged(Muxc::NumberBox, i64),
    RadioButtonsSelectionChanged(Muxc::RadioButtons, i64),
    TextBoxTextChanged(Xaml::TextBox, i64),
    AutoSuggestBoxTextChanged(Xaml::AutoSuggestBox, i64),
    AutoSuggestBoxQuerySubmitted(Xaml::AutoSuggestBox, i64),
    AutoSuggestBoxSuggestionChosen(Xaml::AutoSuggestBox, i64),
    SelectorSelectionChanged(Xaml::Selector, i64),
    NavigationViewSelectionChanged(Muxc::NavigationView, i64),
    NavigationViewBackRequested(Muxc::NavigationView, i64),
    TabViewAddTabButtonClick(Muxc::TabView, i64),
    TabViewCloseRequested(Muxc::TabView, i64),
    TabViewSelectionChanged(Muxc::TabView, i64),
    TeachingTipActionButtonClick(Muxc::TeachingTip, i64),
    TeachingTipClosed(Muxc::TeachingTip, i64),
    RangeBaseValueChanged(Xaml::RangeBase, i64),
    RatingControlValueChanged(Xaml::RatingControl, i64),
    PasswordBoxPasswordChanged(Xaml::PasswordBox, i64),
}

impl Drop for EventSubscription {
    fn drop(&mut self) {
        match self {
            Self::BreadcrumbBarItemClicked(target, token) => {
                let _ = target.RemoveItemClicked(*token);
            }
            Self::ButtonClick(target, token) => {
                let _ = target.RemoveClick(*token);
            }
            Self::ExpanderCollapsed(target, token) => {
                let _ = target.RemoveCollapsed(*token);
            }
            Self::ExpanderExpanding(target, token) => {
                let _ = target.RemoveExpanding(*token);
            }
            Self::InfoBarClosed(target, token) => {
                let _ = target.RemoveClosed(*token);
            }
            Self::ToggleButtonChecked(target, token) => {
                let _ = target.RemoveChecked(*token);
            }
            Self::ToggleButtonUnchecked(target, token) => {
                let _ = target.RemoveUnchecked(*token);
            }
            Self::ToggleSwitchToggled(target, token) => {
                let _ = target.RemoveToggled(*token);
            }
            Self::NumberBoxValueChanged(target, token) => {
                let _ = target.RemoveValueChanged(*token);
            }
            Self::RadioButtonsSelectionChanged(target, token) => {
                let _ = target.RemoveSelectionChanged(*token);
            }
            Self::TextBoxTextChanged(target, token) => {
                let _ = target.RemoveTextChanged(*token);
            }
            Self::AutoSuggestBoxTextChanged(target, token) => {
                let _ = target.RemoveTextChanged(*token);
            }
            Self::AutoSuggestBoxQuerySubmitted(target, token) => {
                let _ = target.RemoveQuerySubmitted(*token);
            }
            Self::AutoSuggestBoxSuggestionChosen(target, token) => {
                let _ = target.RemoveSuggestionChosen(*token);
            }
            Self::SelectorSelectionChanged(target, token) => {
                let _ = target.RemoveSelectionChanged(*token);
            }
            Self::NavigationViewSelectionChanged(target, token) => {
                let _ = target.RemoveSelectionChanged(*token);
            }
            Self::NavigationViewBackRequested(target, token) => {
                let _ = target.RemoveBackRequested(*token);
            }
            Self::TabViewAddTabButtonClick(target, token) => {
                let _ = target.RemoveAddTabButtonClick(*token);
            }
            Self::TabViewCloseRequested(target, token) => {
                let _ = target.RemoveTabCloseRequested(*token);
            }
            Self::TabViewSelectionChanged(target, token) => {
                let _ = target.RemoveSelectionChanged(*token);
            }
            Self::TeachingTipActionButtonClick(target, token) => {
                let _ = target.RemoveActionButtonClick(*token);
            }
            Self::TeachingTipClosed(target, token) => {
                let _ = target.RemoveClosed(*token);
            }
            Self::RangeBaseValueChanged(target, token) => {
                let _ = target.RemoveValueChanged(*token);
            }
            Self::RatingControlValueChanged(target, token) => {
                let _ = target.RemoveValueChanged(*token);
            }
            Self::PasswordBoxPasswordChanged(target, token) => {
                let _ = target.RemovePasswordChanged(*token);
            }
        }
    }
}

fn xaml_thickness(t: Thickness) -> Xaml::Thickness {
    Xaml::Thickness {
        Left: t.left,
        Top: t.top,
        Right: t.right,
        Bottom: t.bottom,
    }
}

fn xaml_color(c: Color) -> Xaml::Color {
    Xaml::Color {
        A: c.a,
        R: c.r,
        G: c.g,
        B: c.b,
    }
}

fn xaml_brush(brush: &Brush) -> Result<Xaml::Brush> {
    match brush {
        Brush::Solid(color) => {
            let b = Xaml::SolidColorBrush::new()?;
            b.SetColor(xaml_color(*color))?;
            b.cast()
        }
    }
}

fn solid_brush(color: Xaml::Color) -> Result<Xaml::Brush> {
    let brush = Xaml::SolidColorBrush::new()?;
    brush.SetColor(color)?;
    brush.cast()
}

fn apply_button_style_variant(button: &Xaml::Button, variant: i32) -> Result<()> {
    let control: Xaml::Control = button.cast()?;
    match variant {
        1 => {
            control.SetBackground(&solid_brush(Xaml::Color {
                A: 255,
                R: 0,
                G: 120,
                B: 212,
            })?)?;
            control.SetForeground(&solid_brush(Xaml::Color {
                A: 255,
                R: 255,
                G: 255,
                B: 255,
            })?)?;
        }
        2 => {
            control.SetBackground(&solid_brush(Xaml::Color {
                A: 0,
                R: 0,
                G: 0,
                B: 0,
            })?)?;
        }
        3 => {
            control.SetBackground(&solid_brush(Xaml::Color {
                A: 0,
                R: 0,
                G: 0,
                B: 0,
            })?)?;
            control.SetForeground(&solid_brush(Xaml::Color {
                A: 255,
                R: 0,
                G: 102,
                B: 204,
            })?)?;
        }
        _ => {}
    }
    Ok(())
}

fn hstring(s: &str) -> windows_core::HSTRING {
    windows_core::HSTRING::from(s)
}

fn string_reference(s: &str) -> IInspectable {
    windows::Foundation::PropertyValue::CreateString(&windows_core::HSTRING::from(s))
        .expect("PropertyValue::CreateString should box string content")
}

fn inspectable_to_string(value: IInspectable) -> Option<String> {
    value
        .cast::<windows::Foundation::IPropertyValue>()
        .ok()
        .and_then(|v| v.GetString().ok())
        .map(|s| s.to_string_lossy())
}

fn set_content_string(handle: &Handle, s: &str) -> Result<()> {
    let tb = Xaml::TextBlock::new()?;
    tb.SetText(&hstring(s))?;
    if let Ok(cc) = handle.cast_inner::<Xaml::ContentControl>() {
        cc.SetContent(&tb)?;
    } else if let Ok(ddb) = handle.cast_inner::<Xaml::DropDownButton>() {
        ddb.SetContent(&string_reference(s))?;
    } else if let Ok(split) = handle.cast_inner::<Xaml::SplitButton>() {
        split.SetContent(&string_reference(s))?;
    }
    Ok(())
}

fn panel_children_vec(parent: &Handle) -> Option<windows_collections::IVector<Xaml::UIElement>> {
    parent
        .cast_inner::<Xaml::Panel>()
        .ok()?
        .Children()
        .ok()?
        .cast()
        .ok()
}

fn content_control_for(parent: &Handle) -> Option<Xaml::ContentControl> {
    parent.cast_inner().ok()
}

fn items_vector(parent: &Handle) -> Option<windows_collections::IVector<IInspectable>> {
    parent
        .cast_inner::<Xaml::ItemsControl>()
        .ok()?
        .Items()
        .ok()?
        .cast()
        .ok()
}

fn set_string_items(handle: &Handle, items: &[String]) -> Result<()> {
    if let Some(vec) = items_vector(handle) {
        vec.Clear()?;
        for item in items {
            vec.Append(&string_reference(item))?;
        }
    }
    Ok(())
}

fn set_radio_buttons_items(radio: &Muxc::RadioButtons, items: &[String]) -> Result<()> {
    let vec = radio.Items()?;
    vec.Clear()?;
    for item in items {
        vec.Append(&string_reference(item))?;
    }
    Ok(())
}

fn set_breadcrumb_items(breadcrumb: &Muxc::BreadcrumbBar, items: &[String]) -> Result<()> {
    let vec: Vec<Option<IInspectable>> = items
        .iter()
        .map(|item| Some(string_reference(item)))
        .collect();
    let vec: windows_collections::IVector<IInspectable> = vec.into();
    breadcrumb.SetItemsSource(&vec)
}

fn set_string_items_for_items_control<T: Interface>(control: &T, items: &[String]) -> Result<()> {
    let items_control: Xaml::ItemsControl = control.cast()?;
    let vec: windows_collections::IVector<IInspectable> = items_control.Items()?.cast()?;
    vec.Clear()?;
    for item in items {
        vec.Append(&string_reference(item))?;
    }
    Ok(())
}

fn set_grid_rows(grid: &Xaml::Grid, rows: &[GridLength]) -> Result<()> {
    let defs = grid.RowDefinitions()?;
    defs.Clear()?;
    for row in rows {
        let def = Xaml::RowDefinition::new()?;
        def.SetHeight(xaml_grid_length(*row))?;
        defs.Append(&def)?;
    }
    Ok(())
}

fn set_grid_columns(grid: &Xaml::Grid, cols: &[GridLength]) -> Result<()> {
    let defs = grid.ColumnDefinitions()?;
    defs.Clear()?;
    for col in cols {
        let def = Xaml::ColumnDefinition::new()?;
        def.SetWidth(xaml_grid_length(*col))?;
        defs.Append(&def)?;
    }
    Ok(())
}

fn xaml_grid_length(length: GridLength) -> Xaml::GridLength {
    match length {
        GridLength::Auto => Xaml::GridLength {
            Value: 0.0,
            GridUnitType: Xaml::GridUnitType::Auto,
        },
        GridLength::Pixel(v) => Xaml::GridLength {
            Value: v,
            GridUnitType: Xaml::GridUnitType::Pixel,
        },
        GridLength::Star(v) => Xaml::GridLength {
            Value: v,
            GridUnitType: Xaml::GridUnitType::Star,
        },
    }
}

fn set_image_uri(image: &Xaml::Image, uri: &str) -> Result<()> {
    if uri.is_empty() {
        return Ok(());
    }
    let uri = windows::Foundation::Uri::CreateUri(&hstring(uri))?;
    let bitmap = Xaml::BitmapImage::new()?;
    bitmap.cast::<Xaml::BitmapImage>()?.SetUriSource(&uri)?;
    image.SetSource(&bitmap.cast::<Xaml::ImageSource>()?)
}

fn set_nav_items(nav: &Muxc::NavigationView, items: &[NavViewItem]) -> Result<()> {
    let vec = nav.MenuItems()?;
    vec.Clear()?;
    for item in items {
        let element = build_nav_item(item)?;
        let inspectable: IInspectable = element.cast()?;
        vec.Append(&inspectable)?;
    }
    Ok(())
}

fn build_nav_item(item: &NavViewItem) -> Result<Muxc::NavigationViewItemBase> {
    if item.is_header {
        let header = Muxc::NavigationViewItemHeader::new()?;
        header.SetContent(&string_reference(&item.content))?;
        return header.cast();
    }

    let nav_item = Muxc::NavigationViewItem::new()?;
    nav_item.SetContent(&string_reference(&item.content))?;
    if let Some(tag) = &item.tag {
        nav_item
            .cast::<Xaml::FrameworkElement>()?
            .SetTag(&string_reference(tag))?;
    }
    if let Some(icon) = item.icon {
        let icon = Xaml::SymbolIcon::CreateInstanceWithSymbol(Xaml::Symbol(icon.0))?;
        nav_item.SetIcon(&icon.cast::<Xaml::IconElement>()?)?;
    }
    nav_item.cast()
}

fn select_nav_tag(nav: &Muxc::NavigationView, tag: &str) -> Result<()> {
    let items = nav.MenuItems()?;
    for i in 0..items.Size()? {
        let item = items.GetAt(i)?;
        if nav_item_has_tag(&item, tag)? {
            nav.SetSelectedItem(&item)?;
            break;
        }
    }
    Ok(())
}

fn nav_item_has_tag(item: &IInspectable, tag: &str) -> Result<bool> {
    if let Ok(fe) = item.cast::<Xaml::FrameworkElement>()
        && let Ok(value) = fe.Tag()
        && inspectable_to_string(value).as_deref() == Some(tag)
    {
        return Ok(true);
    }
    Ok(false)
}
