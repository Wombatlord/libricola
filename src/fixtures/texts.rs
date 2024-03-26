use std::error::Error;

use sqlx::{types::Json, PgPool};

use crate::domain::text::{Metadata, Text};

pub struct TextFixtures;

impl TextFixtures {
    pub async fn create(pool: &PgPool, texts: Vec<Text>) -> Result<(), Box<dyn Error>> {
        let mut txn = pool.begin().await?;
        let sql =
            "INSERT INTO texts (text_type_id, author_id, title, published, metadata) VALUES ($1, $2, $3, $4, $5)";

        for text in texts {
            sqlx::query(sql)
                .bind(&text.text_type_id)
                .bind(&text.author_id)
                .bind(&text.title)
                .bind(&text.published)
                .bind(Json(&text.metadata))
                .execute(&mut *txn)
                .await?;
        }

        txn.commit().await?;
        Ok(())
    }

    pub async fn populate_shakespeare(pool: &PgPool) -> Result<(), Box<dyn Error>> {
        let hamlet = Text::new(
            6,
            1,
            "Hamlet".into(),
            1603,
            Metadata {
                genre_tags: vec!["Tragedy".into()],
            },
        );

        let twelfth = Text::new(
            6,
            1,
            "Twelfth Night".into(),
            1623,
            Metadata {
                genre_tags: vec!["Comedy".into()],
            },
        );

        let tempest = Text::new(
            6,
            1,
            "The Tempest".into(),
            1623,
            Metadata {
                genre_tags: vec!["Comedy".into(), "Tragedy".into()],
            },
        );

        let henry_v = Text::new(
            6,
            1,
            "Henry V".into(),
            1623,
            Metadata {
                genre_tags: vec!["History".into()],
            },
        );

        let lear = Text::new(
            6,
            1,
            "King Lear".into(),
            1608,
            Metadata {
                genre_tags: vec!["Tragedy".into()],
            },
        );

        let texts = vec![hamlet, twelfth, tempest, henry_v, lear];
        TextFixtures::create(pool, texts).await?;

        Ok(())
    }

    pub async fn populate_homer(pool: &PgPool) -> Result<(), Box<dyn Error>> {
        let odyssey = Text::new(
            5,
            2,
            "Odyssey".into(),
            1614,
            Metadata {
                genre_tags: vec!["Epic".into()],
            },
        );

        let illiad = Text::new(
            5,
            2,
            "Illiad".into(),
            1598,
            Metadata {
                genre_tags: vec!["Epic".into()],
            },
        );

        let texts = vec![odyssey, illiad];

        TextFixtures::create(pool, texts).await?;

        Ok(())
    }

    pub async fn populate_eliot(pool: &PgPool) -> Result<(), Box<dyn Error>> {
        let prufrock = Text::new(
            5,
            3,
            "The Love Song of J. Alfred Prufrock".into(),
            1915,
            Metadata {
                genre_tags: vec!["Modernism".into()],
            },
        );

        let wasteland = Text::new(
            5,
            3,
            "The Waste Land".into(),
            1922,
            Metadata {
                genre_tags: vec!["Modernism".into()],
            },
        );

        let texts = vec![prufrock, wasteland];

        TextFixtures::create(pool, texts).await?;

        Ok(())
    }

    pub async fn populate_pynchon(pool: &PgPool) -> Result<(), Box<dyn Error>> {
        let v = Text::new(
            2,
            4,
            "V".into(),
            1963,
            Metadata {
                genre_tags: vec!["Post Modernism".into()],
            },
        );

        let lot_49 = Text::new(
            2,
            4,
            "The Crying of Lot 49".into(),
            1966,
            Metadata {
                genre_tags: vec!["Post Modernism".into(), "Conspiracy".into()],
            },
        );

        let rainbow = Text::new(
            1,
            4,
            "Gravity's Rainbow".into(),
            1973,
            Metadata {
                genre_tags: vec!["Post Modernism".into(), "Conspiracy".into()],
            },
        );

        let vineland = Text::new(
            1,
            4,
            "Vineland".into(),
            1990,
            Metadata {
                genre_tags: vec!["Post Modernism".into()],
            },
        );

        let m_and_d = Text::new(
            1,
            4,
            "Mason & Dixon".into(),
            1997,
            Metadata {
                genre_tags: vec!["Post Modernism".into(), "History".into()],
            },
        );

        let texts = vec![v, lot_49, rainbow, vineland, m_and_d];
        TextFixtures::create(pool, texts).await?;
        Ok(())
    }

    pub async fn populate_banks(pool: &PgPool) -> Result<(), Box<dyn Error>> {
        let phlebas = Text::new(
            1,
            5,
            "Consider Phlebas".into(),
            1987,
            Metadata {
                genre_tags: vec!["Sci-Fi".into()],
            },
        );

        let pog = Text::new(
            1,
            5,
            "The Player of Games".into(),
            1988,
            Metadata {
                genre_tags: vec!["Sci-Fi".into()],
            },
        );

        let detail = Text::new(
            1,
            5,
            "Surface Detail".into(),
            2010,
            Metadata {
                genre_tags: vec!["Sci-Fi".into()],
            },
        );

        let excession = Text::new(
            1,
            5,
            "Excession".into(),
            1996,
            Metadata {
                genre_tags: vec!["Sci-Fi".into()],
            },
        );

        let sonata = Text::new(
            1,
            5,
            "The Hydrogen Sonata".into(),
            2012,
            Metadata {
                genre_tags: vec!["Sci-Fi".into()],
            },
        );

        let texts = vec![phlebas, pog, detail, excession, sonata];
        TextFixtures::create(pool, texts).await?;
        Ok(())
    }
}
