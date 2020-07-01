# flow

## linear.app

[linear.app](https://linear.app/) is a task management product that designed
from two principles:

1. Keyboard navigation
2. Each interaction should be < 100ms (instantaneous)

Both of these serve to make the application feel instant. Viewing all
keyboard navigations can be done by pressing `?`.

### Notifications

Additionally two
other UI elements stand out:

1. A persistent sidebar on the left that anchors all navigation.
2. Feedback for every action through a notification toaster in the bottom-right.

The notification for every action taken is intriguing because it provides
confirmation for actions taken, and often contains a link to the next stage.
For example when adding an issue to a project the confirmation reads:

```txt
1 issue added to "test"

View "test"
```

When clicking "view test" you'll jump straight to the project. This
interaction makes the UI feel responsive, helpful, and helps teach people
about the UI.

### Onboarding

When a new view is opened for the first time, a full-page explainer is shown.
No infinite jumping around as has been the trend for ages. Quick onboarding
for a quick app.
