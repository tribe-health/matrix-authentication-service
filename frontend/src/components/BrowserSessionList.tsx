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

import { graphql, usePaginationFragment } from "react-relay";
import BlockList from "./BlockList";
import BrowserSession from "./BrowserSession";
import Button from "./Button";
import { Title } from "./Typography";

import { BrowserSessionList_user$key } from "./__generated__/BrowserSessionList_user.graphql";

type Props = {
  user: BrowserSessionList_user$key;
  currentSessionId: string;
};

const BrowserSessionList: React.FC<Props> = ({ user, currentSessionId }) => {
  const { data, loadNext, hasNext } = usePaginationFragment(
    graphql`
      fragment BrowserSessionList_user on User
      @refetchable(queryName: "BrowserSessionListQuery") {
        browserSessions(first: $count, after: $cursor)
          @connection(key: "BrowserSessionList_user_browserSessions") {
          edges {
            cursor
            node {
              id
              ...BrowserSession_session
            }
          }
        }
      }
    `,
    user
  );

  return (
    <BlockList>
      <Title>List of browser sessions:</Title>
      {data.browserSessions.edges.map((n) => (
        <BrowserSession
          key={n.cursor}
          session={n.node}
          isCurrent={n.node.id === currentSessionId}
        />
      ))}
      {hasNext && <Button onClick={() => loadNext(2)}>Load more</Button>}
    </BlockList>
  );
};

export default BrowserSessionList;
