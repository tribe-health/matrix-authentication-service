// Copyright 2022 The Matrix.org Foundation C.I.C.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

import { createElement, Children } from "react";

type Variant = "headline" | "title" | "subtitle" | "body" | "caption" | "micro";

type Props = {
  children: React.ReactNode;
  variant: Variant;
  bold?: boolean;
  justified?: boolean;
};

const elementMap: Record<Variant, "h1" | "h2" | "h3" | "p" | "small"> = {
  headline: "h1",
  title: "h2",
  subtitle: "h3",
  body: "p",
  caption: "p",
  micro: "small",
};

const classMap: Record<Variant, string> = {
  headline: "text-3xl font-semibold",
  title: "text-2xl font-semibold",
  subtitle: "text-lg",
  body: "text-base",
  caption: "text-sm",
  micro: "text-xs",
};

const Typography = ({ variant, children, bold, justified }: Props) => {
  const element = elementMap[variant];
  const boldClass = bold ? "font-semibold" : "";
  const justifiedClass = justified ? "text-justify" : "";
  const className = `text-black dark:text-white ${boldClass} ${justifiedClass} ${classMap[variant]}`;
  return createElement(element, { className }, ...Children.toArray(children));
};

type SimpleProps = { children: React.ReactNode };

export const Bold = ({ children }: SimpleProps) => (
  <strong className="font-semibold">{children}</strong>
);

export const Code = ({ children }: SimpleProps) => (
  <code className="font-mono text-sm">{children}</code>
);

export const Title = ({ children }: SimpleProps) => (
  <Typography variant="title" children={children} />
);

export const Subtitle = ({ children }: SimpleProps) => (
  <Typography variant="subtitle" children={children} />
);

export const Body = ({
  children,
  justified,
}: {
  children: React.ReactNode;
  justified?: boolean;
}) => <Typography variant="body" children={children} justified={justified} />;

export default Typography;
