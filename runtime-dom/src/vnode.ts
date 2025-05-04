import { ChildrenTypes, Props } from "./h";

export type VNode = {
  tag: string;
  props: Props;
  children: ChildrenTypes;
  el: HTMLElement | null;
};

export function isVNode(value: any): value is VNode {
  return value ? value.children === true : false;
}
