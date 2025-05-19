use std::path::{Path, PathBuf};
use std::fs;
use std::io;
use serde::{Serialize, Deserialize};
use crate::domain::model::card::{Card, Suit};
use crate::domain::model::game::{Game, GameId, GameVariant, GamePhase, BettingRound};
use crate::domain::model::player::{Player, PlayerId};
use crate::domain::repository::game_repository::GameRepository;
use crate::domain::model::error::DomainError; 