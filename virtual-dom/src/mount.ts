import { isVNode, VNode } from "./vnode";

export function mount(vnode: VNode, container: HTMLElement | null) {
  const el = document.createElement(vnode.tag);
  vnode.el = el;

  for (const key in vnode.props) {
    if (key.startsWith("on") && typeof vnode.props[key] === "function") {
      const eventName = key.slice(2).toLowerCase();
      const event = vnode.props[key];

      el.addEventListener(eventName, event);
    } else if (typeof vnode.props[key] === "string") {
      el.setAttribute(key, vnode.props[key]);
    }

    if (typeof vnode.children === "string") {
      el.textContent = vnode.children;
    } else if (Array.isArray(vnode.children)) {
      vnode.children.forEach((child) => mount(child, el));
    } else if (isVNode(vnode.children)) {
      mount(vnode.children, el);
    }
  }

  container?.appendChild(el);
}

export function unmount(vnode: VNode) {
  if (vnode.el && vnode.el.parentNode) {
    vnode.el.parentNode.removeChild(vnode.el);
  }
}
