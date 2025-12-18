# English translations for AI Todolist Bot

# === Welcome & Start ===
welcome = <b>{ $name }</b>, welcome!

    I'm your AI task assistant.
    Text or voice — I'll understand.

    <code>Call mom tomorrow at 5pm</code>
    Try it now ↑

welcome-trial = 🎁 You have a <b>7-day free trial</b>!
welcome-referral-bonus = 🎉 +7 days bonus from referral!

# === Help ===
help-text = <b>Available commands:</b>

    /tasks — List your tasks
    /today — Today's tasks
    /settings — Settings & subscription
    /invite — Invite friends, get +7 days
    /support — Contact support

    Just send me a message to create a task!

# === Tasks ===
task-created = ✅ Task created: <b>{ $title }</b>
task-created-with-reminder = ✅ Task created: <b>{ $title }</b>
    ⏰ Reminder: { $reminder }
task-done = ✅ Done: <b>{ $title }</b>
task-deleted = 🗑 Deleted: { $title }
task-updated = ✏️ Updated: <b>{ $title }</b>

tasks-empty = 📋 No tasks yet!

    Send me a message to create one.
tasks-header = 📋 Your tasks ({ $count }):
tasks-today-empty = 📅 No tasks for today.

    Enjoy your free time! 🌴
tasks-today-header = 📅 Today's tasks ({ $count }):

task-status-pending = ⏳
task-status-in-progress = 🔄
task-status-done = ✅

# === Task Actions ===
task-snooze-1h = Snoozed for 1 hour
task-snooze-tomorrow = Snoozed until tomorrow
task-reminder-snoozed = ⏰ Reminder snoozed: { $title }

# === Reminders ===
reminder-title = ⏰ <b>Reminder</b>
reminder-task = { $title }
reminder-due = Due: { $due }

# === Morning Brief ===
brief-title = ☀️ <b>Good morning!</b>
brief-tasks-count = You have <b>{ $count }</b> { $count ->
    [one] task
    *[other] tasks
} for today:
brief-no-tasks = No tasks scheduled for today. Have a great day! 🌟
brief-tip = 💡 Send me a voice message to quickly add tasks!

# === Weekly Review ===
weekly-title = 📊 <b>Weekly Review</b>
weekly-completed = ✅ Completed: <b>{ $count }</b> { $count ->
    [one] task
    *[other] tasks
}
weekly-created = 📝 Created: <b>{ $count }</b>
weekly-pending = ⏳ Pending: <b>{ $count }</b>
weekly-great-job = 🎉 Great job this week!
weekly-keep-going = 💪 Keep going!

# === Settings ===
settings-title = ⚙️ <b>Settings</b>
settings-status = 📊 Status: { $status }
settings-timezone = 🌍 Timezone: { $tz }
settings-brief-time = ⏰ Morning brief: { $time }
settings-language = 🌐 Language: { $lang }

settings-timezone-title = 🌍 <b>Select timezone:</b>
settings-brief-title = ⏰ <b>Morning brief time:</b>
settings-language-title = 🌐 <b>Select language:</b>

settings-updated = ✅ Settings updated!
settings-timezone-updated = ✅ Timezone set to { $tz }
settings-brief-updated = ✅ Morning brief set to { $time }
settings-language-updated = ✅ Language changed to { $lang }

# === Subscription ===
subscription-trial = 🎁 Trial ({ $days } { $days ->
    [one] day
    *[other] days
})
subscription-active = ✅ Active ({ $days } { $days ->
    [one] day
    *[other] days
})
subscription-trial-warning = ⚠️ Trial ends in { $days } { $days ->
    [one] day
    *[other] days
}
subscription-expired = ❌ Expired

subscription-expired-message = ⏰ Your trial has ended!

    To continue using AI Todolist, subscribe for just <b>⭐50 Stars/month</b>.

    ✅ Unlimited tasks
    ✅ Voice messages
    ✅ Smart reminders
    ✅ Morning briefs

subscription-success = 🎉 Thank you for subscribing!

    ✅ Your { $type } subscription is now active until { $expires }.

    Enjoy AI Todolist! 🚀

# === Referrals ===
invite-title = 🎁 <b>Invite Friends</b>
invite-description = Share your link and get <b>+7 days</b> for each friend who joins!
invite-link = Your referral link:
invite-stats = 👥 Invited: <b>{ $count }</b>
    🎁 Bonus days: <b>{ $bonus }</b>
invite-share = 📲 Share:

# === Support ===
support-prompt = 📮 <b>Contact Support</b>

    Send your message below and we'll get back to you soon.
support-sent = ✅ Message sent to support. We'll reply soon!

# === Stale Tasks ===
stale-warning = ⚠️ { $count } { $count ->
    [one] task hasn't
    *[other] tasks haven't
} been updated in 7+ days.

    Consider reviewing or completing them!

# === Duplicate Detection ===
duplicate-warning = ⚠️ Similar task exists:
    <i>{ $title }</i>

    Create anyway?

# === Celebration ===
celebration-streak = 🔥 <b>{ $count }-task streak!</b> Keep it up!
celebration-milestone = 🏆 <b>Milestone!</b> You've completed { $count } tasks!

# === Voice ===
voice-processing = 🎤 Processing voice message...
voice-transcribed = 📝 Transcribed: <i>{ $text }</i>

# === Draft ===
draft-created = 📝 <b>Message draft:</b>

    { $text }

    <i>Copy and send when ready!</i>

# === Vague Task ===
vague-suggestion = 💡 This task seems broad. Want to break it down?

    Suggestions:
    { $suggestions }

# === Tags ===
tag-work = 💼 Work
tag-personal = 🏠 Personal
tag-shopping = 🛒 Shopping
tag-health = 🏥 Health
tag-finance = 💰 Finance
tag-other = 📌 Other

# === Buttons ===
btn-done = ✅ Done
btn-delete = 🗑 Delete
btn-edit = ✏️ Edit
btn-snooze-1h = ⏰ 1h
btn-snooze-tomorrow = 📅 Tomorrow
btn-back = ← Back
btn-timezone = 🌍 Timezone
btn-brief-time = ⏰ Brief time
btn-language = 🌐 Language
btn-invite = 🎁 Invite friends
btn-subscribe = ⭐ Subscribe
btn-create-anyway = ✅ Create anyway
btn-cancel = ❌ Cancel
btn-yes = ✅ Yes
btn-no = ❌ No

# === Errors ===
error-generic = ❌ Something went wrong. Please try again.
error-not-found = ❌ Task not found.
error-subscription-required = ⭐ Subscription required to use this feature.

# === Admin ===
admin-title = 🔧 <b>Admin Panel</b>
admin-stats = 📊 <b>Statistics</b>
admin-users = 👥 <b>Users</b>
admin-broadcast = 📢 <b>Broadcast</b>
