export function h(tag, props, children) {
  return { tag, props, children };
}

export function unmount(vnode) {
  vnode.el.parentNode.removeChild(vnode.el);
}
export function mount(vnode, container) {
  const el = document.createElement(vnode.tag);
  vnode.el = el;

  for (const key in vnode.props) {
    if (key.startsWith("on")) {
      const eventName = key.slice(2).toLowerCase();

      el.addEventListener(eventName, vnode.props[key]);
    } else {
      el.setAttribute(key, vnode.props[key]);
    }
  }

  if (typeof vnode.children === "string") {
    el.textContent = vnode.children;
  } else if (Array.isArray(vnode.children)) {
    vnode.children.forEach((child) => mount(child, el));
  } else {
    mount(vnode.children, el);
  }

  container.appendChild(el);
}
export function patch(vnode1, vnode2) {
  if (typeof vnode2.children === "string") {
    vnode1.el.textContent = vnode2.children;
  } else {
    for (let i = 0; i < vnode1.children.length; i++) {
      patch(vnode1.children[i], vnode2.children[i]);
    }
  }
}
