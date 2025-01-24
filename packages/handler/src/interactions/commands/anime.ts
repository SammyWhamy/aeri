import {
    ActionRowBuilder,
    SlashCommandBuilder,
    StringSelectMenuBuilder,
    StringSelectMenuOptionBuilder,
} from "@discordjs/builders";
import { env } from "core";
import { ApplicationCommandOptionType } from "discord-api-types/v10";
import { Logger } from "log";
import type { Command } from "../../services/commands.js";
import { getCommandOption } from "../../utility/interactionUtils.js";

const logger = new Logger();
export const interaction: Command = {
    cooldown: 10,
    data: new SlashCommandBuilder()
        .setName("anime")
        .setDescription("Find An Anime")
        .addStringOption((option) =>
            option.setName("media_name").setDescription("Name Of The Anime").setRequired(true),
        ),
    async execute(interaction): Promise<void> {
        const anime = getCommandOption("media_name", ApplicationCommandOptionType.String, interaction.options) || "";

        const response = await fetch(`${env.API_URL}/relations`, {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({
                media_name: anime,
                media_type: "ANIME",
            }),
        }).catch((error) => {
            logger.error("Error while fetching data from the API.", "Anilist", error);
            return null;
        });

        if (!response) {
            logger.error("Request returned null", "Anilist");
            return interaction.reply({ content: "Problem trying to fetch data", ephemeral: true });
        }

        const result = await response.json().catch((error) => {
            logger.error("Error while parsing JSON data.", "Anilist", error);
            return interaction.reply({ content: "Problem trying to fetch data", ephemeral: true });
        });

        if (!result) {
            return interaction.reply({ content: "Problem trying to fetch data", ephemeral: true });
        }

        if (result.relations.length === 0) {
            return interaction.reply({ content: "No relations were found", ephemeral: true });
        }

        const select = new StringSelectMenuBuilder()
            .setCustomId("media_selection:anime")
            .setPlaceholder("Choose A Media...")
            .setMinValues(1)
            .setMaxValues(1)
            .addOptions(
                result.relations.slice(0, 25).map((items: { native: any; english: any; romaji: any; id: any }) => {
                    return new StringSelectMenuOptionBuilder()
                        .setLabel(
                            `${items.english === null ? (items.native || items.romaji || "").slice(0, 100) : items.romaji.slice(0, 100)}`,
                        )
                        .setValue(`${items.id}`)
                        .setDescription(
                            `${items.english === null ? (items.native || items.romaji || "").slice(0, 100) : items.romaji.slice(0, 100)}`,
                        );
                }),
            );
        const row = new ActionRowBuilder().addComponents(select);
        await interaction.reply({ components: [row] });
    },
};
