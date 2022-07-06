use anchor_lang::prelude::*;

// pub mod bid;
pub mod mint;
pub mod sell;

// use bid::*;
use mint::*;
use sell::*;

declare_id!("55VX31JFN1H5UAxHTbpWE2xEMoJSm8DznJorY8S4mkAp");
// account verification(uname(immutable), display name, image url)
// post creation (uri, imgUrl, title, post, owner, creator, type(buyable, bidable, not for sale)) (uer must have account)
// bid and buy
// claim bid award
// like reward claim

#[program]
pub mod social_blocks {

use super::*;

  pub fn mint(
    ctx: Context<MintNft>,
    metadata_title: String, 
    metadata_symbol: String, 
    metadata_uri: String,
    kind: u8,
    price: u64
  ) -> Result<()> {
      mint::mint(
        ctx,
        metadata_title,
        metadata_symbol,
        metadata_uri,
        kind,
        price
      )
  }

  pub fn sell(
    ctx: Context<SellNft>,
    sale_lamports: u64
  ) -> Result<()> {
    sell::sell(
      ctx,
      sale_lamports,
    )
  }

  // pub fn bid(
  //   ctx: Context<BidNft>,
  //   bid_lamports: u64,
  // ) -> Result<()> {
  //   bid::bid(
  //     ctx,
  //     bid_lamports,
  //   )
  // }
}
