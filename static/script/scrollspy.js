const headings_json = `@{HEADINGS_JSON}`;

if (!!headings_json) {
    const headings = JSON.parse(headings_json);
    let template = document.createElement('template');
    const navbar_object = traverse_headings(headings)

    const html = render_nav_tree(navbar_object);
    template.innerHTML = `<ul id="post-toc-nav" class="h-100 list-group">${html}</ul>`

    document.querySelector('#post-toc').appendChild(template.content);

}

function render_nav_tree(tree) {
    if (tree.children.length > 0) {
        let html = ""
        if ('id' in tree) {
            html = `<ul class="list-group list-group-flush"><a href="#${tree['id']}">${tree['content']}</a>`;
        } else {
            html = '<ul class="list-group list-group-flush">'
        }
        for (let child of tree.children) {
            html += render_nav_tree(child);
        }
        html += '</ul>';
        return html;
    } else {
        return `<li class="list-group-item border border-0"><a class="ms-1" href="#${tree['id']}">${tree['content']}</a></li>`
    }
}


function traverse_headings(headings_list) {
    if (!headings_list || headings_list.length === 0) return [];
    const retval = {
        children: []
    };
    let last_node = retval;
    let parent_node = retval;
    let grandpa_node = void 0;
    let current_level = 0;
    while (headings_list.length > 0) {
        const current_heading = headings_list.splice(0, 1)[0];
        let tag = current_heading['tag']
        let id = current_heading['id']
        let content = current_heading['content']
        let level_num = parseInt(tag.replace('h', ''))
        const heading = {
            level: level_num,
            id: id,
            content: content,
            children: [],
            parent: void 0
        };


        if (level_num === current_level) {
            // Sibling heading
            heading.parent = parent_node;
            parent_node.children.push(heading);
        } else if (level_num < current_level) {
            // Parent's sibling
            heading.parent = grandpa_node
            grandpa_node.children.push(heading)
            parent_node = grandpa_node
            grandpa_node = parent_node.parent
        } else {
            // Child
            last_node.children.push(heading);
            heading.parent = last_node;
            parent_node = last_node
            grandpa_node = parent_node.parent;
        }
        current_level = level_num;
        last_node = heading;
    }
    return retval;
}