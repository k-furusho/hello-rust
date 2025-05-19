#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ベット額_作成と値取得() {
        let bet = BetAmount::new(100);
        assert_eq!(bet.value(), 100);
    }

    #[test]
    fn ベット額_ゼロ生成と判定() {
        let bet = BetAmount::zero();
        assert_eq!(bet.value(), 0);
        assert!(bet.is_zero());
        
        let non_zero = BetAmount::new(10);
        assert!(!non_zero.is_zero());
    }

    #[test]
    fn ベット額_加算() {
        let bet1 = BetAmount::new(50);
        let bet2 = BetAmount::new(30);
        let result = bet1.add(bet2);
        assert_eq!(result.value(), 80);
    }

    #[test]
    fn ベット額_減算_成功() {
        let bet1 = BetAmount::new(50);
        let bet2 = BetAmount::new(30);
        let result = bet1.subtract(bet2).unwrap();
        assert_eq!(result.value(), 20);
    }

    #[test]
    fn ベット額_減算_失敗() {
        let bet1 = BetAmount::new(30);
        let bet2 = BetAmount::new(50);
        let result = bet1.subtract(bet2);
        assert!(result.is_err());
    }

    #[test]
    fn チップ_作成と値取得() {
        let chips = Chips::new(500);
        assert_eq!(chips.amount(), 500);
    }

    #[test]
    fn チップ_加算() {
        let mut chips = Chips::new(100);
        chips.add(50);
        assert_eq!(chips.amount(), 150);
    }

    #[test]
    fn チップ_ベット額加算() {
        let mut chips = Chips::new(100);
        let bet = BetAmount::new(50);
        chips.add_bet_amount(bet);
        assert_eq!(chips.amount(), 150);
    }

    #[test]
    fn チップ_減算_成功() {
        let mut chips = Chips::new(100);
        let result = chips.subtract(50);
        assert!(result.is_ok());
        assert_eq!(chips.amount(), 50);
    }

    #[test]
    fn チップ_減算_失敗() {
        let mut chips = Chips::new(30);
        let result = chips.subtract(50);
        assert!(result.is_err());
        assert_eq!(chips.amount(), 30); // 変更されていないことを確認
    }

    #[test]
    fn チップ_ゼロ判定() {
        let chips_zero = Chips::new(0);
        assert!(chips_zero.is_zero());
        
        let chips_non_zero = Chips::new(10);
        assert!(!chips_non_zero.is_zero());
    }

    #[test]
    fn ポット_作成と合計取得() {
        let mut pot = Pot::new();
        assert_eq!(pot.total(), 0);
        
        pot.add(100);
        assert_eq!(pot.total(), 100);
        assert_eq!(pot.main_pot(), 100);
    }

    #[test]
    fn ポット_ベット額追加() {
        let mut pot = Pot::new();
        let bet = BetAmount::new(150);
        pot.add_bet(bet);
        assert_eq!(pot.total(), 150);
    }

    #[test]
    fn ポット_サイドポット作成() {
        let mut pot = Pot::new();
        pot.add(500);
        assert_eq!(pot.main_pot(), 500);
        
        // サイドポットを作成
        let player_ids = vec!["player1".to_string(), "player2".to_string()];
        pot.create_side_pot(200, player_ids.clone());
        
        // メインポットは減少し、サイドポットが作成される
        assert_eq!(pot.main_pot(), 300);
        assert_eq!(pot.side_pots().len(), 1);
        let (side_pot_chips, side_pot_players) = &pot.side_pots()[0];
        assert_eq!(side_pot_chips.amount(), 200);
        assert_eq!(side_pot_players, &player_ids);
    }

    #[test]
    fn ポット_クリア() {
        let mut pot = Pot::new();
        pot.add(300);
        pot.create_side_pot(100, vec!["player1".to_string()]);
        
        assert_eq!(pot.total(), 300);
        assert_eq!(pot.side_pots().len(), 1);
        
        pot.clear();
        assert_eq!(pot.total(), 0);
        assert_eq!(pot.side_pots().len(), 0);
    }
} 