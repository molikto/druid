use crate::text::{
    movement, offset_for_delete_backwards, EditAction, EditableText, Movement,
    Selection
};

/// Insert text at the cursor position.
/// Replaces selected text if there's a selection.
pub fn insert(selection: &mut Selection, src: &mut String, new: &str) {
    // EditableText's edit method will panic if selection is greater than
    // src length, hence we try to constrain it.
    //
    // This is especially needed when data was modified externally.
    // TODO: perhaps this belongs in update?
    *selection = selection.constrain_to(src);

    src.edit(selection.range(), new);
    *selection = Selection::caret(selection.min() + new.len());
}

/// Set the selection to be a caret at the given offset, if that's a valid
/// codepoint boundary.
pub fn caret_to(selection: &mut Selection, text: &mut String, to: usize) {
    match text.cursor(to) {
        Some(_) => *selection = Selection::caret(to),
        None => log::error!("You can't move the cursor there."),
    }
}

pub fn do_edit_action(selection: &mut Selection, edit_action: EditAction, text: &mut String) {
    match edit_action {
        EditAction::Insert(chars) | EditAction::Paste(chars) => insert(selection, text, &chars),
        EditAction::Backspace => delete_backward(selection, text),
        EditAction::Delete => delete_forward(selection, text),
        EditAction::JumpDelete(movement) => {
            move_selection(selection, movement, text, true);
            delete_forward(selection, text)
        }
        EditAction::JumpBackspace(movement) => {
            move_selection(selection, movement, text, true);
            delete_backward(selection, text)
        }
        EditAction::Move(movement) => move_selection(selection, movement, text, false),
        EditAction::ModifySelection(movement) => move_selection(selection, movement, text, true),
        EditAction::SelectAll => selection.all(text),
        EditAction::Click(action) => {
            if action.mods.shift() {
                selection.end = action.column;
            } else {
                caret_to(selection, text, action.column);
            }
        }
        EditAction::Drag(action) => selection.end = action.column,
    }
}

/// Edit a selection using a `Movement`.
pub fn move_selection(selection: &mut Selection, mvmnt: Movement, text: &mut String, modify: bool) {
    // This movement function should ensure all movements are legit.
    // If they aren't, that's a problem with the movement function.
    *selection = movement(mvmnt, *selection, text, modify);
}

/// Delete to previous grapheme if in caret mode.
/// Otherwise just delete everything inside the selection.
pub fn delete_backward(selection: &mut Selection, text: &mut String) {
    if selection.is_caret() {
        let cursor = selection.end;
        let new_cursor = offset_for_delete_backwards(selection, text);
        text.edit(new_cursor..cursor, "");
        caret_to(selection, text, new_cursor);
    } else {
        text.edit(selection.range(), "");
        caret_to(selection, text, selection.min());
    }
}

pub fn delete_forward(selection: &mut Selection, text: &mut String) {
    if selection.is_caret() {
        // Never touch the characters before the cursor.
        if text.next_grapheme_offset(selection.end).is_some() {
            move_selection(selection, Movement::Right, text, false);
            delete_backward(selection, text);
        }
    } else {
        delete_backward(selection, text);
    }
}
