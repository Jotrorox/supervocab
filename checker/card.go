package checker

import (
	"encoding/json"
	"time"
)

type Root struct {
	Items map[string]Item `json:"-"`
}

type Item struct {
	Data       Data       `json:"data"`
	Membership Membership `json:"membership"`
	Backlinks  []string   `json:"backlinks"`
	Parents    Parents    `json:"parents"`
}

type Data struct {
	ID               string    `json:"id"`
	OwnerID          string    `json:"owner_id"`
	Name             string    `json:"name"`
	Markup           string    `json:"markup,omitempty"`
	HTML             string    `json:"html,omitempty"`
	YDoc             string    `json:"ydoc"`
	Icon             string    `json:"icon,omitempty"`
	Tags             []string  `json:"tags"`
	Color            *string   `json:"color,omitempty"`
	CreatedWhen      time.Time `json:"created_when"`
	ModifiedWhen     time.Time `json:"modified_when"`
	ModifiedByID     string    `json:"modified_by_id"`
	SyncedWhen       time.Time `json:"synced_when"`
	Meta             Meta      `json:"meta"`
	TargetedWhen     *string   `json:"targeted_when,omitempty"`
	Likes            int       `json:"likes"`
	MemberCount      int       `json:"member_count"`
	CommentCount     int       `json:"comment_count"`
	PublicChildCount int       `json:"public_child_count"`
}

type Membership struct {
	ID                  string    `json:"id"`
	Liked               *string   `json:"liked,omitempty"`
	PersonalTags        []string  `json:"personal_tags"`
	PersonalColor       *string   `json:"personal_color,omitempty"`
	Perms               int       `json:"perms"`
	ViaType             int       `json:"via_type"`
	ViaID               *string   `json:"via_id,omitempty"`
	CreatedWhen         time.Time `json:"created_when"`
	ModifiedWhen        time.Time `json:"modified_when"`
	EnrolledWhen        time.Time `json:"enrolled_when"`
	OpenedWhen          *string   `json:"opened_when,omitempty"`
	AutoPublishChildren bool      `json:"auto_publish_children"`
	View                *string   `json:"view,omitempty"`
	Visibility          int       `json:"visibility"`
	Status              int       `json:"status"`
	TotalChildCount     int       `json:"total_child_count"`
	ShareLinkCount      int       `json:"share_link_count"`
}

type Meta struct{}

type Parents struct{}

func (r *Root) jsonToCard(data []byte) error {
	var temp map[string]json.RawMessage
	if err := json.Unmarshal(data, &temp); err != nil {
		return err
	}

	r.Items = make(map[string]Item)
	for key, value := range temp {
		var item Item
		if err := json.Unmarshal(value, &item); err != nil {
			return err
		}
		r.Items[key] = item
	}

	return nil
}
