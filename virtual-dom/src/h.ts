import { type VNode } from "./index";

export type Props = Record<string, string | (() => void)> | null;

export type ChildrenTypes =
  | string
  | VNode
  | ((
      tag: string,
      props: Props,
      children: ChildrenTypes | ChildrenTypes[]
    ) => VNode);

export function h(
  tag: string,
  props: Props,
  children: ChildrenTypes | ChildrenTypes[]
): VNode;

export function h(tag: any, props: any, children: any) {
  return { tag, props, children };
}
