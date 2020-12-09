/**
 * Copyright 2020 Garrit Franke
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      https://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
use crate::lexer::Keyword;
use crate::lexer::{Token, TokenKind};
use crate::parser::node_type::*;
use crate::util::string_util::highlight_position_in_file;
use std::convert::TryFrom;
use std::iter::Peekable;
use std::vec::IntoIter;

pub struct Parser {
    tokens: Peekable<IntoIter<Token>>,
    peeked: Vec<Token>,
    current: Option<Token>,
    prev: Option<Token>,
    raw: Option<String>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>, raw: Option<String>) -> Parser {
        let tokens_without_whitespace: Vec<Token> = tokens
            .into_iter()
            .filter(|token| token.kind != TokenKind::Whitespace && token.kind != TokenKind::Comment)
            .collect();
        Parser {
            tokens: tokens_without_whitespace.into_iter().peekable(),
            peeked: vec![],
            current: None,
            prev: None,
            raw: raw,
        }
    }

    pub fn parse(&mut self) -> Result<Program, String> {
        self.parse_program()
    }

    pub(super) fn next(&mut self) -> Result<Token, String> {
        self.prev = self.current.to_owned();
        let item = if self.peeked.is_empty() {
            self.tokens.next()
        } else {
            self.peeked.pop()
        };

        self.current = item.to_owned();
        item.ok_or_else(|| "Expected token".into())
    }

    pub(super) fn peek(&mut self) -> Result<Token, String> {
        let token = self.next()?;
        self.push(token.to_owned());
        Ok(token)
    }

    pub(super) fn drop(&mut self, count: usize) {
        for _ in 0..count {
            let _ = self.next();
        }
    }

    pub(super) fn push(&mut self, token: Token) {
        self.peeked.push(token);
    }

    pub(super) fn has_more(&mut self) -> bool {
        !self.peeked.is_empty() || self.tokens.peek().is_some()
    }

    pub(super) fn match_token(&mut self, token_kind: TokenKind) -> Result<Token, String> {
        match self.next()? {
            token if token.kind == token_kind => Ok(token),
            other => Err(self.make_error(token_kind, other)),
        }
    }

    pub(super) fn peek_token(&mut self, token_kind: TokenKind) -> Result<Token, String> {
        match self.peek()? {
            token if token.kind == token_kind => Ok(token),
            other => Err(format!(
                "Token {:?} not found, found {:?}",
                token_kind, other
            )),
        }
    }

    pub(super) fn match_keyword(&mut self, keyword: Keyword) -> Result<(), String> {
        let token = self.next()?;
        match &token.kind {
            TokenKind::Keyword(ref k) if k == &keyword => Ok(()),
            _ => Err(self.make_error(TokenKind::SemiColon, token)),
        }
    }

    pub(super) fn match_operator(&mut self) -> Result<BinOp, String> {
        BinOp::try_from(self.next()?.kind)
    }
    pub(super) fn match_identifier(&mut self) -> Result<String, String> {
        match self.next()?.kind {
            TokenKind::Identifier(n) => Ok(n),
            other => Err(format!("Expected Identifier, found {:?}", other)),
        }
    }

    pub(super) fn make_error(&mut self, token_kind: TokenKind, other: Token) -> String {
        match &self.raw {
            Some(raw_file) => format!(
                "Token {:?} not found, found {:?}\n{:?}",
                token_kind,
                other,
                highlight_position_in_file(raw_file.to_string(), other.to_owned().pos)
            ),
            None => format!("Token {:?} not found, found {:?}", token_kind, other),
        }
    }

    pub(super) fn prev(&mut self) -> Option<Token> {
        self.prev.clone()
    }
}
